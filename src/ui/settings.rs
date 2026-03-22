// F3: Settings panel screen

use iced::{
    widget::{button, column, container, row, text, text_input, toggler},
    Alignment, Element, Length, Task,
};

use crate::config::{self, Settings, Theme};

#[derive(Debug, Clone)]
pub struct SettingsScreen {
    settings: Settings,
    status_input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ThemeToggled,
    NotificationsToggled(bool),
    SoundToggled(bool),
    FontSizeIncreased,
    FontSizeDecreased,
    StatusInputChanged(String),
    Back,
}

impl SettingsScreen {
    pub fn new(settings: Settings) -> Self {
        Self {
            status_input: settings.status_message.clone().unwrap_or_default(),
            settings,
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ThemeToggled => {
                self.settings.theme = match self.settings.theme {
                    Theme::Dark => Theme::Light,
                    Theme::Light => Theme::Dark,
                };
                let _ = config::save(&self.settings);
                Task::none()
            }
            Message::NotificationsToggled(enabled) => {
                self.settings.notifications_enabled = enabled;
                let _ = config::save(&self.settings);
                Task::none()
            }
            Message::SoundToggled(enabled) => {
                self.settings.sound_enabled = enabled;
                let _ = config::save(&self.settings);
                Task::none()
            }
            Message::FontSizeIncreased => {
                if self.settings.font_size < 20 {
                    self.settings.font_size += 1;
                    let _ = config::save(&self.settings);
                }
                Task::none()
            }
            Message::FontSizeDecreased => {
                if self.settings.font_size > 12 {
                    self.settings.font_size -= 1;
                    let _ = config::save(&self.settings);
                }
                Task::none()
            }
            Message::StatusInputChanged(value) => {
                self.status_input = value.clone();
                self.settings.status_message = if self.status_input.trim().is_empty() {
                    None
                } else {
                    Some(self.status_input.trim().to_string())
                };
                let _ = config::save(&self.settings);
                Task::none()
            }
            Message::Back => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("Settings").size(20);

        let is_dark = self.settings.theme == Theme::Dark;
        let theme_row = row![
            text("Dark theme").size(14).width(Length::Fill),
            toggler(is_dark).on_toggle(|_| Message::ThemeToggled),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let notif_row = row![
            text("Notifications").size(14).width(Length::Fill),
            toggler(self.settings.notifications_enabled).on_toggle(Message::NotificationsToggled),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let sound_row = row![
            text("Sound").size(14).width(Length::Fill),
            toggler(self.settings.sound_enabled).on_toggle(Message::SoundToggled),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let font_row = row![
            text(format!("Font size: {}", self.settings.font_size))
                .size(14)
                .width(Length::Fill),
            button("-")
                .on_press(Message::FontSizeDecreased)
                .padding([4, 10]),
            button("+")
                .on_press(Message::FontSizeIncreased)
                .padding([4, 10]),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let status_row = row![
            text("Status:").size(14).width(80),
            text_input("e.g. In a meeting", &self.status_input)
                .on_input(Message::StatusInputChanged)
                .width(Length::Fill),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let back_btn = button("Back").on_press(Message::Back).padding([6, 14]);

        let content =
            column![title, theme_row, notif_row, sound_row, font_row, status_row, back_btn]
                .spacing(16)
                .padding(24)
                .width(400);

        container(content)
            .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
