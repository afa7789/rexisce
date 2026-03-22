.PHONY: setup build run test lint fmt fmt-check clean

# First-time setup: ensure stable toolchain is up to date
setup:
	rustup update stable

# Compile a release binary
build:
	cargo build --release

# Run the app in debug mode
run:
	cargo run

# Run the app in release mode
run-release:
	cargo build --release && ./target/release/xmpp-start

# Run the full test suite (single-threaded — required for SQLite tests)
test:
	cargo test --bin xmpp-start -- --test-threads=1

# Run integration tests only
test-integration:
	cargo test --test critical_flows -- --test-threads=1

# Lint with clippy (warnings are errors)
lint:
	cargo clippy --bin xmpp-start -- -D warnings

# Auto-format all source files
fmt:
	cargo fmt

# Check formatting without modifying files (used in CI)
fmt-check:
	cargo fmt --check

# Remove build artifacts
clean:
	cargo clean
