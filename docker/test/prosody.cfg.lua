-- Test-only Prosody config.
-- Self-signed TLS cert so tokio-xmpp STARTTLS works.

c2s_require_encryption = false
s2s_require_encryption = false
authentication = "internal_plain"
storage = "internal"
allow_registration = false  -- accounts created by entrypoint.sh

-- Self-signed certificate for STARTTLS
ssl = {
    certificate = "/etc/prosody/certs/localhost.crt";
    key = "/etc/prosody/certs/localhost.key";
}

log = { info = "*console"; warn = "*console"; error = "*console" }

VirtualHost "localhost"
    ssl = {
        certificate = "/etc/prosody/certs/localhost.crt";
        key = "/etc/prosody/certs/localhost.key";
    }

modules_enabled = {
    "roster";
    "saslauth";
    "tls";
    "disco";
    "carbons";
    "smacks";
    "mam";
    "ping";
    "posix";
}

archive_expires_after = "1d"
default_archive_policy = true
