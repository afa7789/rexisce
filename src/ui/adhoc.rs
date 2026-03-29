// L4: Ad-hoc commands UI — XEP-0050
//
// Flow:
//   1. User opens panel → app sends XmppCommand::DiscoverAdhocCommands { target_jid }
//   2. Engine returns XmppEvent::AdhocCommandsDiscovered { from_jid, commands }
//      → populate command list
//   3. User selects a command → app sends XmppCommand::ExecuteAdhocCommand { to_jid, node }
//   4. Engine returns XmppEvent::AdhocCommandResult(CommandResponse)
//      → show response form (using XEP-0004 DataForm renderer)
//   5. User fills form and submits → app sends XmppCommand::ContinueAdhocCommand
//   6. Or user cancels → app sends XmppCommand::CancelAdhocCommand

use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Element, Length,
};

use crate::ui::data_forms::{render_form, render_form_interactive, DataForm, FieldType, FormField};
use crate::xmpp::modules::adhoc::{CommandResponse, CommandStatus, DataField};

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Which step of the ad-hoc flow we are in.
#[derive(Debug, Clone, PartialEq)]
pub enum AdhocStep {
    /// Showing the server JID input and waiting for user to confirm discovery.
    TargetInput,
    /// Waiting for discovery results to arrive.
    Discovering,
    /// Command list is shown; user has not yet selected one.
    CommandList,
    /// A command was selected and executed; we are waiting for the result.
    Executing,
    /// Showing a form returned by the server.
    ShowingForm(CommandResponse),
    /// Command completed — holds summary message and optional read-only result form.
    Done(String, Option<DataForm>),
}

#[derive(Debug, Clone)]
pub struct AdhocScreen {
    /// JID of the server/service to discover commands on.
    pub target_jid: String,
    /// Available commands: (node, name).
    pub commands: Vec<(String, String)>,
    /// Current step.
    pub step: AdhocStep,
    /// Form field values (keyed by `var`).
    pub field_values: std::collections::HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// Messages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Message {
    /// User typed in the target JID field.
    TargetJidChanged(String),
    /// User clicked "Discover".
    DiscoverRequested,
    /// Engine returned the command list.
    CommandsDiscovered {
        _from_jid: String,
        commands: Vec<(String, String)>,
    },
    /// User clicked on a command item.
    CommandSelected(String),
    /// Engine returned a command response.
    CommandResponseReceived(CommandResponse),
    /// User changed a form field.
    FieldChanged(String, String),
    /// User clicked "Submit" on the form.
    SubmitForm,
    /// User clicked "Cancel" on the form.
    CancelCommand,
    /// User clicked "Close" / back.
    Close,
    /// User clicked "Back to list" after a command completes.
    BackToList,
}

// ---------------------------------------------------------------------------
// Actions
// ---------------------------------------------------------------------------

pub enum Action {
    None,
    Close,
    Discover {
        target_jid: String,
    },
    Execute {
        target_jid: String,
        node: String,
    },
    Submit {
        target_jid: String,
        node: String,
        session_id: String,
        fields: Vec<DataField>,
    },
    Cancel {
        target_jid: String,
        node: String,
        session_id: String,
    },
}

// ---------------------------------------------------------------------------
// Implementation
// ---------------------------------------------------------------------------

impl Default for AdhocScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl AdhocScreen {
    pub fn new() -> Self {
        Self {
            target_jid: String::new(),
            commands: vec![],
            step: AdhocStep::TargetInput,
            field_values: std::collections::HashMap::new(),
        }
    }

    /// The node of the currently-executing command (if any).
    pub fn active_node(&self) -> Option<&str> {
        match &self.step {
            AdhocStep::ShowingForm(resp) => Some(&resp.node),
            _ => None,
        }
    }

    /// The session_id of the current command response (if any).
    pub fn active_session_id(&self) -> Option<&str> {
        match &self.step {
            AdhocStep::ShowingForm(resp) => Some(&resp.session_id),
            _ => None,
        }
    }

    /// Collect the current form fields as DataField snapshots for submission.
    pub fn collect_fields(&self) -> Vec<DataField> {
        match &self.step {
            AdhocStep::ShowingForm(resp) => resp
                .fields
                .iter()
                .map(|f| DataField {
                    var: f.var.clone(),
                    label: f.label.clone(),
                    field_type: f.field_type.clone(),
                    value: self.field_values.get(&f.var).cloned().or(f.value.clone()),
                    options: f.options.clone(),
                })
                .collect(),
            _ => vec![],
        }
    }

    /// Convert engine `DataField`s (from a `CommandResponse`) into a `DataForm`
    /// suitable for rendering with `render_form_interactive`.
    fn data_fields_to_data_form(fields: &[DataField]) -> DataForm {
        let form_fields: Vec<FormField> = fields
            .iter()
            .map(|f| {
                let field_type = match f.field_type.as_str() {
                    "text-private" => FieldType::TextPrivate,
                    "text-multi" => FieldType::TextMulti,
                    "boolean" => FieldType::Boolean,
                    "list-single" => FieldType::ListSingle,
                    "list-multi" => FieldType::ListMulti,
                    "fixed" => FieldType::Fixed,
                    "hidden" => FieldType::Hidden,
                    "jid-single" => FieldType::JidSingle,
                    "jid-multi" => FieldType::JidMulti,
                    _ => FieldType::TextSingle,
                };
                FormField {
                    var: Some(f.var.clone()),
                    field_type,
                    label: f.label.clone(),
                    value: f.value.clone(),
                    required: false,
                    options: f.options.clone(),
                }
            })
            .collect();

        DataForm {
            title: None,
            instructions: None,
            fields: form_fields,
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::TargetJidChanged(v) => {
                self.target_jid = v;
                Action::None
            }
            Message::DiscoverRequested => {
                let target = self.target_jid.clone();
                self.step = AdhocStep::Discovering;
                self.commands.clear();
                Action::Discover { target_jid: target }
            }
            Message::CommandsDiscovered {
                _from_jid: _,
                commands,
            } => {
                self.commands = commands;
                self.step = AdhocStep::CommandList;
                Action::None
            }
            Message::CommandSelected(node) => {
                let target = self.target_jid.clone();
                self.step = AdhocStep::Executing;
                self.field_values.clear();
                Action::Execute {
                    target_jid: target,
                    node,
                }
            }
            Message::CommandResponseReceived(resp) => {
                match resp.status {
                    CommandStatus::Completed => {
                        let notes = resp.notes.join("; ");
                        let summary = if notes.is_empty() {
                            "Command completed.".into()
                        } else {
                            notes
                        };
                        // Build a read-only result form if the server returned fields.
                        let result_form = if resp.fields.is_empty() {
                            None
                        } else {
                            Some(Self::data_fields_to_data_form(&resp.fields))
                        };
                        self.step = AdhocStep::Done(summary, result_form);
                    }
                    CommandStatus::Canceled => {
                        self.step = AdhocStep::CommandList;
                    }
                    CommandStatus::Executing => {
                        self.field_values.clear();
                        for f in &resp.fields {
                            if let Some(ref v) = f.value {
                                self.field_values.insert(f.var.clone(), v.clone());
                            }
                        }
                        self.step = AdhocStep::ShowingForm(resp);
                    }
                }
                Action::None
            }
            Message::FieldChanged(var, val) => {
                self.field_values.insert(var, val);
                Action::None
            }
            Message::SubmitForm => {
                if let Some(node) = self.active_node().map(str::to_owned) {
                    if let Some(session_id) = self.active_session_id().map(str::to_owned) {
                        let fields = self.collect_fields();
                        return Action::Submit {
                            target_jid: self.target_jid.clone(),
                            node,
                            session_id,
                            fields,
                        };
                    }
                }
                Action::None
            }
            Message::CancelCommand => {
                let action = if let Some(node) = self.active_node().map(str::to_owned) {
                    if let Some(session_id) = self.active_session_id().map(str::to_owned) {
                        Action::Cancel {
                            target_jid: self.target_jid.clone(),
                            node,
                            session_id,
                        }
                    } else {
                        Action::None
                    }
                } else {
                    Action::None
                };
                self.step = AdhocStep::CommandList;
                self.field_values.clear();
                action
            }
            Message::Close => Action::Close,
            Message::BackToList => {
                self.step = AdhocStep::CommandList;
                self.field_values.clear();
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Ad-Hoc Commands (XEP-0050)").size(20);
        let close_btn = button("Close").on_press(Message::Close).padding([4, 12]);

        let body: Element<'_, Message> = match &self.step {
            AdhocStep::TargetInput => self.view_target_input(),
            AdhocStep::Discovering => text("Discovering commands\u{2026}").into(),
            AdhocStep::CommandList => self.view_command_list(),
            AdhocStep::Executing => text("Executing command\u{2026}").into(),
            AdhocStep::ShowingForm(resp) => self.view_form(resp),
            AdhocStep::Done(msg, result_form) => {
                let mut done_col = column![text(msg.as_str())].spacing(8);
                // If the server returned result fields, show them as a read-only form.
                if let Some(form) = result_form.clone() {
                    done_col = done_col.push(render_form::<Message>(form));
                }
                done_col = done_col.push(
                    button("Back to list")
                        .on_press(Message::BackToList)
                        .padding([4, 12]),
                );
                done_col.into()
            }
        };

        let content = column![
            row![title, close_btn].spacing(8).align_y(Alignment::Center),
            body,
        ]
        .spacing(16)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_target_input(&self) -> Element<'_, Message> {
        let jid_input = text_input("admin@example.com or server.example.com", &self.target_jid)
            .on_input(Message::TargetJidChanged)
            .on_submit(Message::DiscoverRequested)
            .padding(6)
            .width(Length::Fixed(320.0));
        let discover_btn = button("Discover")
            .on_press(Message::DiscoverRequested)
            .padding([6, 12]);
        column![
            text("Enter the JID of the server or service:"),
            row![jid_input, discover_btn].spacing(8),
        ]
        .spacing(8)
        .into()
    }

    fn view_command_list(&self) -> Element<'_, Message> {
        if self.commands.is_empty() {
            return column![
                text("No commands found on this service."),
                button("Try again")
                    .on_press(Message::DiscoverRequested)
                    .padding([4, 12]),
            ]
            .spacing(8)
            .into();
        }

        let items: Vec<Element<'_, Message>> = self
            .commands
            .iter()
            .map(|(node, name)| {
                let label = if name.is_empty() {
                    node.as_str()
                } else {
                    name.as_str()
                };
                button(text(label))
                    .on_press(Message::CommandSelected(node.clone()))
                    .width(Length::Fill)
                    .padding([6, 12])
                    .into()
            })
            .collect();

        let list =
            scrollable(column(items).spacing(4).width(Length::Fill)).height(Length::Fixed(300.0));

        column![
            text(format!("{} command(s) available:", self.commands.len())),
            list,
        ]
        .spacing(8)
        .into()
    }

    fn view_form(&self, resp: &CommandResponse) -> Element<'static, Message> {
        let form = Self::data_fields_to_data_form(&resp.fields);
        let field_values = self.field_values.clone();
        let form_el = render_form_interactive(form, &field_values, Message::FieldChanged);

        // Notes from the server.
        let mut notes_col = column![].spacing(4);
        for note in &resp.notes {
            notes_col = notes_col.push(text(note.clone()).size(12));
        }

        let submit_btn = button("Submit")
            .on_press(Message::SubmitForm)
            .padding([6, 12]);
        let cancel_btn = button("Cancel")
            .on_press(Message::CancelCommand)
            .padding([6, 12]);

        let col = column![form_el, notes_col, row![submit_btn, cancel_btn].spacing(8),].spacing(8);

        scrollable(col).height(Length::Fixed(400.0)).into()
    }
}
