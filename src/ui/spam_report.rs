// Task L5 — Spam reporting UI (XEP-0377)
//
// A simple modal dialog that lets the user report a JID as a spammer.
// The dialog is pre-filled with the target JID and has an optional reason
// text area.  It produces a `SpamReportCommand` that the caller converts
// into `XmppCommand::ReportSpam`.

use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Element, Length,
};

// ---------------------------------------------------------------------------
// Command returned to the caller
// ---------------------------------------------------------------------------

/// The side-effect the caller should perform after the user submits.
#[derive(Debug, Clone)]
pub struct SpamReportCommand {
    pub jid: String,
    pub reason: Option<String>,
}

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct SpamReportModal {
    /// JID being reported — pre-filled from context.
    pub jid: String,
    /// Optional free-text reason entered by the user.
    pub reason: String,
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    JidChanged(String),
    ReasonChanged(String),
    Submit,
    Cancel,
}

// ---------------------------------------------------------------------------
// Impl
// ---------------------------------------------------------------------------

impl SpamReportModal {
    /// Create a new modal pre-filled with `jid`.
    pub fn new(jid: impl Into<String>) -> Self {
        Self {
            jid: jid.into(),
            reason: String::new(),
        }
    }

    /// Update state.  Returns `Some(SpamReportCommand)` when the user
    /// submits, or `None` (including when they cancel — the caller must
    /// close the modal on `Message::Cancel` independently).
    pub fn update(&mut self, msg: Message) -> Option<SpamReportCommand> {
        match msg {
            Message::JidChanged(v) => {
                self.jid = v;
                None
            }
            Message::ReasonChanged(v) => {
                self.reason = v;
                None
            }
            Message::Submit => {
                let jid = self.jid.trim().to_string();
                if jid.is_empty() {
                    return None;
                }
                let reason = self.reason.trim().to_string();
                Some(SpamReportCommand {
                    jid,
                    reason: if reason.is_empty() {
                        None
                    } else {
                        Some(reason)
                    },
                })
            }
            Message::Cancel => None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Report Spam").size(16);

        let jid_input = text_input("JID to report", &self.jid)
            .on_input(Message::JidChanged)
            .padding([6, 10])
            .width(Length::Fill);

        let reason_input = text_input("Reason (optional)", &self.reason)
            .on_input(Message::ReasonChanged)
            .on_submit(Message::Submit)
            .padding([6, 10])
            .width(Length::Fill);

        let submit_btn = button(text("Report").size(13))
            .on_press(Message::Submit)
            .padding([6, 12]);

        let cancel_btn = button(text("Cancel").size(13))
            .on_press(Message::Cancel)
            .padding([6, 12]);

        let buttons = row![cancel_btn, submit_btn]
            .spacing(8)
            .align_y(Alignment::Center);

        let content = column![title, jid_input, reason_input, buttons].spacing(12);

        container(content).padding(16).into()
    }
}
