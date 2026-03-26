//! Subscription helpers — each function produces one `iced::Subscription<Message>`
//! that the main `App::subscription()` batches together.

use iced::Subscription;

use super::chat;
use super::Message;
use crate::xmpp::{AccountId, XmppEvent};

/// Type alias matching the one in `mod.rs`.
type MultiEventRx =
    std::sync::Arc<std::sync::Mutex<Option<tokio::sync::mpsc::Receiver<(AccountId, XmppEvent)>>>>;

/// S1: periodic idle tick — fires every 30 s so App can check auto-away conditions.
pub fn idle_tick() -> Subscription<Message> {
    iced::time::every(std::time::Duration::from_secs(30)).map(|_| Message::IdleTick)
}

/// F2: keyboard shortcuts — Cmd+K palette, Cmd+V paste, Cmd+B bold, Cmd+I italic,
/// Escape toggle palette, Tab / Shift+Tab focus cycle.
pub fn keyboard_shortcuts() -> Subscription<Message> {
    iced::keyboard::on_key_press(|key, modifiers| {
        use iced::keyboard::Key;
        if modifiers.command() {
            if key == Key::Character("k".into()) {
                return Some(Message::TogglePalette);
            }
            if key == Key::Character("v".into()) {
                return Some(Message::PasteFromClipboard);
            }
            if key == Key::Character("b".into()) {
                return Some(Message::Chat(chat::Message::ComposerBold));
            }
            if key == Key::Character("i".into()) {
                return Some(Message::Chat(chat::Message::ComposerItalic));
            }
        }
        if key == Key::Named(iced::keyboard::key::Named::Escape) {
            return Some(Message::TogglePalette);
        }
        if key == Key::Named(iced::keyboard::key::Named::Tab) {
            return if modifiers.shift() {
                Some(Message::FocusPrevious)
            } else {
                Some(Message::FocusNext)
            };
        }
        None
    })
}

/// I2: file-drop subscription — emits `FilesDropped` when a file is dropped on the window.
pub fn file_drop() -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _id| {
        use iced::Event;
        if let Event::Window(iced::window::Event::FileDropped(path)) = event {
            return Some(Message::FilesDropped(vec![path]));
        }
        None
    })
}

/// M4: periodic voice tick — fires every second to update the elapsed timer.
pub fn voice_tick() -> Subscription<Message> {
    iced::time::every(std::time::Duration::from_secs(1))
        .map(|_| Message::Chat(chat::Message::VoiceTick))
}

/// DC-21: multi-engine event subscription — polls the shared receiver for events
/// from additional account engines and routes them as `XmppEvent`.
pub fn multi_engine_events(multi_rx: MultiEventRx) -> Subscription<Message> {
    Subscription::run_with_id("multi-engine-events", {
        iced::stream::channel(32, move |mut output| async move {
            // Take the receiver out of the shared slot (once).
            let mut rx = {
                let mut guard = multi_rx
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                guard.take()
            };
            if let Some(ref mut rx) = rx {
                while let Some((_account_id, event)) = rx.recv().await {
                    let _ = output.try_send(Message::XmppEvent(event));
                }
            }
            // Keep the future alive so the subscription isn't dropped.
            std::future::pending::<()>().await;
        })
    })
}
