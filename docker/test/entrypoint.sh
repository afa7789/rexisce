#!/bin/sh
set -e

echo "[test-server] starting Prosody..."

# Verify config is readable
if [ ! -f /etc/prosody/prosody.cfg.lua ]; then
    echo "[test-server] ERROR: /etc/prosody/prosody.cfg.lua not found!" >&2
    exit 1
fi

# Verify TLS cert exists
if [ ! -f /etc/prosody/certs/localhost.crt ]; then
    echo "[test-server] WARNING: no TLS cert found, Prosody may fail STARTTLS" >&2
fi

# Start Prosody in foreground briefly to catch startup errors, then background it.
prosody &
PROSODY_PID=$!

# Wait until it responds to status checks (max 30s).
TRIES=0
until prosodyctl status 2>/dev/null; do
    TRIES=$((TRIES + 1))
    if [ "$TRIES" -gt 60 ]; then
        echo "[test-server] ERROR: Prosody did not start within 30s" >&2
        prosodyctl status 2>&1 || true
        exit 1
    fi
    sleep 0.5
done

# Create fixed test accounts.
prosodyctl register alice localhost alice123 2>/dev/null || true
prosodyctl register bob   localhost bob123   2>/dev/null || true

echo "[test-server] ready — alice@localhost / bob@localhost"

# Keep the container alive by waiting on the Prosody process.
wait $PROSODY_PID
