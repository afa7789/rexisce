// Presence stanza handler (P1.4)
// Extracted from engine.rs to keep file size manageable.

use tokio::sync::mpsc;
use tokio_xmpp::minidom::Element;
use tokio_xmpp::parsers::presence::{Presence, Type as PresenceType};

use crate::xmpp::XmppEvent;

pub(crate) async fn handle_presence(el: Element, event_tx: &mpsc::Sender<XmppEvent>) {
    let presence = match Presence::try_from(el) {
        Ok(p) => p,
        Err(_) => return,
    };

    let jid = match presence.from {
        Some(ref f) => f.to_string(),
        None => return,
    };

    let available = !matches!(
        presence.type_,
        PresenceType::Unavailable | PresenceType::Error
    );

    let _ = event_tx
        .send(XmppEvent::PresenceUpdated { jid, available })
        .await;
}
