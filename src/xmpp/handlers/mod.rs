// Stanza dispatch handlers — extracted from engine.rs for maintainability.

pub mod iq;
pub mod message;
pub mod presence;

pub(crate) const NS_RECEIPTS: &str = "urn:xmpp:receipts";
pub(crate) const NS_CHAT_MARKERS: &str = "urn:xmpp:chat-markers:0";

pub(crate) use iq::{
    handle_iq, has_omemo_encrypted, omemo_check_prekey_rotation, omemo_encrypt_and_send,
    omemo_try_decrypt, OmemoEncryptError,
};
pub(crate) use message::handle_message;
pub(crate) use presence::handle_presence;
