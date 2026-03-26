-- Migration 0005: Add fingerprint column for OMEMO device trust management (BTBV)
--
-- Stores the hex-encoded Curve25519 identity key fingerprint per device,
-- used for BTBV (Blind Trust Before Verification) trust decisions.

ALTER TABLE omemo_devices ADD COLUMN fingerprint TEXT;
