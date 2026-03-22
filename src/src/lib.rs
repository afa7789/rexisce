// Library crate root — exposes internal modules for integration tests.
// The binary (main.rs) continues to own the iced application entry point.
pub mod config;
pub mod i18n;
pub mod notifications;
pub mod store;
pub mod ui;
pub mod xmpp;
