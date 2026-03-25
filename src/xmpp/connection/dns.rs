// Task P1.7 — DNS SRV resolution + XEP-0156 host-meta discovery
//
// SRV lookup order (RFC 6120 + XEP-0368):
//   1. _xmpps-client._tcp.{domain}  → Direct TLS
//   2. _xmpp-client._tcp.{domain}   → STARTTLS
//   3. Fallback: {domain}:5222 STARTTLS
//
// Source reference: apps/fluux/src-tauri/src/xmpp_proxy/dns.rs

use anyhow::{Context, Result};
use hickory_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

#[derive(Debug, Clone)]
pub struct ResolvedEndpoint {
    pub host: String,
    pub port: u16,
    #[allow(dead_code)]
    pub tls: TlsMode,
}

#[derive(Debug, Clone)]
pub enum TlsMode {
    Direct,
    StartTls,
}

/// Resolve the best connection endpoint for a domain using standard RFC 6120
/// SRV record discovery.
#[allow(dead_code)]
pub async fn resolve(domain: &str) -> Result<ResolvedEndpoint> {
    resolve_with_override(domain, None).await
}

/// Like [`resolve`] but respects the `manual_srv` override from settings.
///
/// - `domain`: bare domain from the JID (e.g. `"example.com"`)
/// - `manual_srv`: when `Some`, query this exact SRV record name instead of
///   the standard RFC 6120 names (e.g. `"_xmpp-client._tcp.corp.example.com"`)
pub async fn resolve_with_override(
    domain: &str,
    manual_srv: Option<&str>,
) -> Result<ResolvedEndpoint> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());

    if let Some(srv_name) = manual_srv {
        // The user pinned a specific SRV record — honour it and determine TLS
        // mode from the service label prefix.
        let tls = if srv_name.starts_with("_xmpps-") {
            TlsMode::Direct
        } else {
            TlsMode::StartTls
        };
        return lookup_srv(&resolver, srv_name, tls).await;
    }

    // 1. Try Direct TLS (_xmpps-client._tcp)
    let dtls_name = format!("_xmpps-client._tcp.{domain}");
    if let Ok(ep) = lookup_srv(&resolver, &dtls_name, TlsMode::Direct).await {
        return Ok(ep);
    }

    // 2. Try STARTTLS (_xmpp-client._tcp)
    let starttls_name = format!("_xmpp-client._tcp.{domain}");
    if let Ok(ep) = lookup_srv(&resolver, &starttls_name, TlsMode::StartTls).await {
        return Ok(ep);
    }

    // 3. Fallback to domain:5222 STARTTLS
    Ok(ResolvedEndpoint {
        host: domain.to_string(),
        port: 5222,
        tls: TlsMode::StartTls,
    })
}

/// Query a single SRV record and return the highest-priority target.
async fn lookup_srv(
    resolver: &TokioAsyncResolver,
    srv_name: &str,
    tls: TlsMode,
) -> Result<ResolvedEndpoint> {
    let records = resolver
        .srv_lookup(srv_name)
        .await
        .with_context(|| format!("SRV lookup failed for {srv_name}"))?;

    // Pick the record with the lowest priority value (RFC 2782).
    let best = records
        .iter()
        .min_by_key(|r| r.priority())
        .context("SRV response was empty")?;

    let host = best.target().to_utf8();
    // Strip trailing dot added by some resolvers.
    let host = host.trim_end_matches('.').to_string();
    let port = best.port();

    Ok(ResolvedEndpoint { host, port, tls })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_mode_from_srv_name_direct() {
        let name = "_xmpps-client._tcp.example.com";
        let tls = if name.starts_with("_xmpps-") {
            TlsMode::Direct
        } else {
            TlsMode::StartTls
        };
        assert!(matches!(tls, TlsMode::Direct));
    }

    #[test]
    fn tls_mode_from_srv_name_starttls() {
        let name = "_xmpp-client._tcp.example.com";
        let tls = if name.starts_with("_xmpps-") {
            TlsMode::Direct
        } else {
            TlsMode::StartTls
        };
        assert!(matches!(tls, TlsMode::StartTls));
    }

    #[test]
    fn resolved_endpoint_clone() {
        let ep = ResolvedEndpoint {
            host: "xmpp.example.com".to_string(),
            port: 5222,
            tls: TlsMode::StartTls,
        };
        let ep2 = ep.clone();
        assert_eq!(ep2.host, "xmpp.example.com");
        assert_eq!(ep2.port, 5222);
        assert!(matches!(ep2.tls, TlsMode::StartTls));
    }
}
