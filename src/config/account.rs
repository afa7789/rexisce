// MULTI: Account configuration model
//
// Each entry represents one XMPP account the user has configured.
// Passwords are stored in the OS keychain; only a reference key is persisted here.

use serde::{Deserialize, Serialize};

use super::default_true;

/// Returns `true` if `jid` is a minimally valid bare JID: `local@domain`.
///
/// Rules enforced:
/// - Exactly one `@` character.
/// - Non-empty local part (before `@`).
/// - Non-empty domain part (after `@`) containing at least one `.`.
pub fn is_valid_jid(jid: &str) -> bool {
    let Some((local, domain)) = jid.split_once('@') else {
        return false;
    };
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    // Allow "localhost" and IP addresses for local testing
    if domain == "localhost" || domain == "127.0.0.1" {
        return true;
    }
    domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
}

/// Optional proxy configuration for routing an account's connection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProxyConfig {
    /// SOCKS5 / HTTP proxy host.
    pub host: String,
    /// Proxy port.
    pub port: u16,
}

/// Configuration for a single XMPP account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// Bare JID for this account (e.g. "user@example.com").
    pub jid: String,
    /// Key used to look up the password in the OS keychain.
    /// Typically equals `jid`, but stored separately so it can be rotated.
    pub password_key: String,
    /// Whether this account should be connected on startup.
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// Optional proxy for this account's XMPP connection.
    #[serde(default)]
    pub proxy: Option<ProxyConfig>,
    /// Accent colour used in the UI to distinguish accounts (e.g. "#4A90D9").
    #[serde(default)]
    pub color: Option<String>,
}

impl AccountConfig {
    /// Construct a minimal account config (enabled, no proxy, no colour).
    pub fn new(jid: impl Into<String>) -> Self {
        let jid = jid.into();
        let password_key = jid.clone();
        Self {
            jid,
            password_key,
            enabled: true,
            proxy: None,
            color: None,
        }
    }

    /// Validate the configuration, returning an error message if invalid.
    ///
    /// Currently checks that `jid` is a well-formed bare JID (`local@domain`).
    #[allow(dead_code)]
    pub fn validate(&self) -> Result<(), String> {
        if is_valid_jid(&self.jid) {
            Ok(())
        } else {
            Err(format!(
                "Invalid JID \"{}\": must be in the form user@domain",
                self.jid
            ))
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_valid_jid -------------------------------------------------------

    #[test]
    fn valid_jid_accepted() {
        assert!(is_valid_jid("alice@example.com"));
        assert!(is_valid_jid("user@xmpp.server.org"));
        assert!(is_valid_jid("a@b.c"));
    }

    #[test]
    fn jid_without_at_rejected() {
        assert!(!is_valid_jid("nodomainsign"));
        assert!(!is_valid_jid(""));
    }

    #[test]
    fn jid_empty_local_rejected() {
        assert!(!is_valid_jid("@example.com"));
    }

    #[test]
    fn jid_localhost_accepted() {
        assert!(is_valid_jid("user@localhost"));
        assert!(is_valid_jid("alice@127.0.0.1"));
    }

    #[test]
    fn jid_domain_leading_trailing_dot_rejected() {
        assert!(!is_valid_jid("user@.example.com"));
        assert!(!is_valid_jid("user@example.com."));
    }

    // --- AccountConfig::validate --------------------------------------------

    #[test]
    fn validate_ok_for_valid_jid() {
        let a = AccountConfig::new("alice@example.com");
        assert!(a.validate().is_ok());
    }

    #[test]
    fn validate_err_for_invalid_jid() {
        let mut a = AccountConfig::new("alice@example.com");
        a.jid = "notajid".into();
        let err = a.validate().unwrap_err();
        assert!(err.contains("notajid"));
    }

    // --- existing tests -----------------------------------------------------

    #[test]
    fn account_config_defaults() {
        let a = AccountConfig::new("alice@example.com");
        assert_eq!(a.jid, "alice@example.com");
        assert_eq!(a.password_key, "alice@example.com");
        assert!(a.enabled);
        assert!(a.proxy.is_none());
        assert!(a.color.is_none());
    }

    #[test]
    fn account_config_round_trip_json() {
        let a = AccountConfig {
            jid: "bob@xmpp.org".into(),
            password_key: "bob@xmpp.org".into(),
            enabled: false,
            proxy: Some(ProxyConfig {
                host: "proxy.corp.com".into(),
                port: 1080,
            }),
            color: Some("#FF5733".into()),
        };

        let json = serde_json::to_string(&a).unwrap();
        let b: AccountConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(b.jid, "bob@xmpp.org");
        assert!(!b.enabled);
        let proxy = b.proxy.unwrap();
        assert_eq!(proxy.host, "proxy.corp.com");
        assert_eq!(proxy.port, 1080);
        assert_eq!(b.color.as_deref(), Some("#FF5733"));
    }

    #[test]
    fn account_config_missing_optional_fields_deserialize() {
        // Old JSON without `proxy` / `color` / `enabled` must still parse.
        let json = r#"{"jid":"carol@test.net","password_key":"carol@test.net"}"#;
        let c: AccountConfig = serde_json::from_str(json).unwrap();
        assert_eq!(c.jid, "carol@test.net");
        assert!(c.enabled); // default_true
        assert!(c.proxy.is_none());
        assert!(c.color.is_none());
    }
}
