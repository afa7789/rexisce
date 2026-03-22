# xmpp-start

Native XMPP desktop messenger — pure Rust using [iced](https://github.com/iced-rs/iced).

> Built with vibecoding, using other XMPP clients as reference (Halloy, Dino, Gajim).

---

## Requirements

- [Rust 1.80+](https://rustup.rs) — `rustup update stable`
- Linux: `sudo apt install libdbus-1-dev pkg-config libssl-dev`
- macOS / Windows: no extra deps

---

## Getting started

```bash
git clone https://github.com/owner/xmpp-start
cd xmpp-start
make run
```

That's it. The first run compiles everything and opens the app.

---

## Common commands

```bash
make setup          # update Rust toolchain
make run            # compile (debug) and run
make build          # compile release binary → target/release/xmpp-start
make run-release    # compile release and run
make test           # run the full test suite
make lint           # clippy (warnings = errors)
make fmt            # auto-format source files
make clean          # delete build artifacts
```

For verbose XMPP logging:

```bash
RUST_LOG=xmpp_start=debug make run
```

---

## Integration tests

The `tests/critical_flows.rs` file covers end-to-end flows across modules (login → connect, MAM catchup, presence transitions, blocking, etc.):

```bash
make test-integration
```

---

## Stack

| Layer | Crate |
|---|---|
| GUI | iced 0.13 |
| XMPP | tokio-xmpp + xmpp-parsers |
| Async | tokio |
| TLS | rustls |
| Storage | SQLite via sqlx |
| Keychain | keyring |
| i18n | fluent-rs |
| Notifications | notify-rust |

---

## Features

- XMPP login (SASL PLAIN / SCRAM-SHA-256, STARTTLS / Direct TLS)
- Contact roster with presence indicators
- 1:1 chat with message history (XEP-0313 MAM)
- Group chat / MUC (XEP-0045) with roles, moderation, bookmarks
- Message corrections (XEP-0308), retractions (XEP-0424), reactions (XEP-0444)
- File upload (XEP-0363) with image thumbnails
- Avatars (XEP-0084 + vCard-temp)
- Stream Management / session resumption (XEP-0198)
- Message Carbons (XEP-0280)
- Entity Capabilities (XEP-0115) + Service Discovery (XEP-0030)
- Blocking (XEP-0191), Ad-Hoc Commands (XEP-0050)
- Dark / light theme, i18n (en-US, pt-BR)
- macOS, Linux, Windows

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
