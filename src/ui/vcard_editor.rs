// K2: vCard editor UI — XEP-0054 / XEP-0292
//
// A panel for editing the logged-in user's own vCard fields:
//   nickname, full name, organisation, email, phone.
//
// Flow:
//   1. Open panel → app sends XmppCommand::FetchOwnVCard
//   2. Engine returns XmppEvent::OwnVCardReceived(fields) → populate form
//   3. User edits and clicks Save → app sends XmppCommand::SetOwnVCard(fields)
//   4. Engine returns XmppEvent::OwnVCardSaved → show success toast

use iced::{
    widget::{button, column, container, row, text, text_input},
    Alignment, Element, Length, Task,
};

use crate::xmpp::modules::vcard_edit::VCardFields;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct VCardEditorScreen {
    /// Whether we are currently waiting for a get result from the server.
    pub loading: bool,
    /// Whether a save is in progress.
    pub saving: bool,
    /// Current form field values.
    pub fields: VCardFields,
    /// Feedback message shown after save (success / error).
    pub status_msg: Option<String>,
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    /// User clicked "Fetch" or the panel opened — request own vCard.
    FetchRequested,
    /// Engine returned the current vCard.
    VCardLoaded(VCardFields),
    /// Engine confirmed the vCard was saved.
    VCardSaved,
    /// Individual field edits.
    NicknameChanged(String),
    FullNameChanged(String),
    OrgChanged(String),
    EmailChanged(String),
    PhoneChanged(String),
    /// User clicked "Save".
    SaveRequested,
    /// User clicked "Close" / "Back".
    Close,
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl Default for VCardEditorScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl VCardEditorScreen {
    pub fn new() -> Self {
        Self {
            loading: false,
            saving: false,
            fields: VCardFields::default(),
            status_msg: None,
        }
    }

    /// Return the current field snapshot (used by the caller to build SetOwnVCard).
    pub fn current_fields(&self) -> VCardFields {
        self.fields.clone()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::FetchRequested => {
                self.loading = true;
                self.status_msg = None;
            }
            Message::VCardLoaded(fields) => {
                self.loading = false;
                self.fields = fields;
            }
            Message::VCardSaved => {
                self.saving = false;
                self.status_msg = Some("Saved successfully.".into());
            }
            Message::NicknameChanged(v) => self.fields.nickname = v,
            Message::FullNameChanged(v) => self.fields.full_name = v,
            Message::OrgChanged(v) => self.fields.organisation = v,
            Message::EmailChanged(v) => self.fields.email = v,
            Message::PhoneChanged(v) => self.fields.phone = v,
            Message::SaveRequested => {
                self.saving = true;
                self.status_msg = None;
                // Caller intercepts this to send XmppCommand::SetOwnVCard.
            }
            Message::Close => {
                // Caller handles navigation.
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Edit Profile (vCard)").size(20);

        let row_width = Length::Fixed(400.0);
        let label_width = Length::Fixed(120.0);

        let mk_row = |label: &'static str,
                      value: &str,
                      msg: fn(String) -> Message|
         -> Element<'_, Message> {
            row![
                text(label).width(label_width),
                text_input(label, value)
                    .on_input(msg)
                    .width(row_width)
                    .padding(6),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
            .into()
        };

        let nickname_row = mk_row("Nickname", &self.fields.nickname, Message::NicknameChanged);
        let fullname_row = mk_row(
            "Full name",
            &self.fields.full_name,
            Message::FullNameChanged,
        );
        let org_row = mk_row(
            "Organisation",
            &self.fields.organisation,
            Message::OrgChanged,
        );
        let email_row = mk_row("Email", &self.fields.email, Message::EmailChanged);
        let phone_row = mk_row("Phone", &self.fields.phone, Message::PhoneChanged);

        // Loading / saving indicators.
        let info: Element<'_, Message> = if self.loading {
            text("Loading…").into()
        } else if self.saving {
            text("Saving…").into()
        } else if let Some(ref msg) = self.status_msg {
            text(msg.as_str()).into()
        } else {
            text("").into()
        };

        let save_btn = button("Save")
            .on_press(Message::SaveRequested)
            .padding([6, 16]);
        let fetch_btn = button("Refresh")
            .on_press(Message::FetchRequested)
            .padding([6, 12]);
        let close_btn = button("Close").on_press(Message::Close).padding([6, 12]);

        let btn_row = row![save_btn, fetch_btn, close_btn].spacing(8);

        let content = column![
            title,
            nickname_row,
            fullname_row,
            org_row,
            email_row,
            phone_row,
            info,
            btn_row,
        ]
        .spacing(12)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
