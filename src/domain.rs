use anyhow::Result;
use hickory_resolver::TokioResolver;
use serde::Serialize;
use std::net::IpAddr;
use uuid::Uuid;

use crate::AppState;

#[derive(Serialize)]
pub struct VerifyResult {
    pub verified: bool,
    pub domain: Option<String>,
    pub expected_ip: String,
    pub resolved_ips: Vec<String>,
    pub message: String,
}

pub async fn verify_domain(state: &AppState, store_id: Uuid) -> Result<VerifyResult> {
    let expected: IpAddr = state.ip_address.parse()?;

    let domain: Option<String> =
        sqlx::query_scalar("SELECT lower(custom_domain) FROM stores WHERE id = $1")
            .bind(store_id)
            .fetch_optional(&state.db)
            .await?
            .flatten();

    let Some(domain) = domain else {
        return Ok(VerifyResult {
            verified: false,
            domain: None,
            expected_ip: expected.to_string(),
            resolved_ips: vec![],
            message: "No custom domain set for this store.".into(),
        });
    };

    let resolver = TokioResolver::builder_tokio()?.build();
    let resolved: Vec<IpAddr> = match resolver.lookup_ip(domain.as_str()).await {
        Ok(lookup) => lookup.iter().collect(),
        Err(_) => vec![],
    };

    let resolved_ips: Vec<String> = resolved.iter().map(|ip| ip.to_string()).collect();

    if resolved.is_empty() {
        return Ok(VerifyResult {
            verified: false,
            domain: Some(domain),
            expected_ip: expected.to_string(),
            resolved_ips,
            message: "Domain does not resolve yet. DNS changes can take a while to propagate."
                .into(),
        });
    }

    if !resolved.contains(&expected) {
        return Ok(VerifyResult {
            verified: false,
            domain: Some(domain),
            expected_ip: expected.to_string(),
            resolved_ips,
            message: "Domain resolves to a different IP address.".into(),
        });
    }

    sqlx::query(
        "UPDATE stores SET domain_verified_at = now(), updated_at = now() WHERE id = $1 AND lower(custom_domain) = $2",
    )
    .bind(store_id)
    .bind(&domain)
    .execute(&state.db)
    .await?;

    Ok(VerifyResult {
        verified: true,
        domain: Some(domain),
        expected_ip: expected.to_string(),
        resolved_ips,
        message: "Domain verified.".into(),
    })
}
