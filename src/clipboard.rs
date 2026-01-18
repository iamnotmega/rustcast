//! This has all the logic regarding the cliboard history
use arboard::ImageData;
use iced::{
    Alignment, Background,
    Length::Fill,
    border::Radius,
    widget::{Button, Row, Text, container},
};

use crate::{
    app::Message,
    commands::Function,
    config::Theme as ConfigTheme,
    styles::{tint, with_alpha},
};

/// The kinds of clipboard content that rustcast can handle and their contents
#[derive(Debug, Clone)]
pub enum ClipBoardContentType {
    Text(String),
    Image(ImageData<'static>),
}

impl ClipBoardContentType {
    /// Returns the iced element for rendering the clipboard item
    pub fn render_clipboard_item(
        &self,
        theme: ConfigTheme,
    ) -> impl Into<iced::Element<'_, Message>> {
        let (title, subtitle) = match self {
            ClipBoardContentType::Text(text) => (text.clone(), String::new()),
            ClipBoardContentType::Image(_) => ("Image".to_string(), "<img>".to_string()),
        };

        let text_block = iced::widget::Column::new()
            .spacing(2)
            .push(
                Text::new(title)
                    .font(theme.font())
                    .size(16)
                    .color(theme.text_color(1.0)),
            )
            .push(
                Text::new(subtitle)
                    .font(theme.font())
                    .size(13)
                    .color(theme.text_color(0.55)),
            );

        let row = Row::new()
            .spacing(12)
            .align_y(Alignment::Center)
            .width(Fill)
            .height(56)
            .push(container(text_block).width(Fill));

        let theme_clone = theme.clone();
        let content = Button::new(row)
            .on_press(Message::RunFunction(Function::CopyToClipboard(
                self.to_owned(),
            )))
            .style(move |_, _| {
                let mut s = iced::widget::button::Style::default();

                s.text_color = theme_clone.text_color(1.0);
                s
            })
            .width(Fill)
            .height(56);

        container(content)
            .style(move |_| iced::widget::container::Style {
                background: Some(Background::Color(with_alpha(
                    tint(theme.bg_color(), 0.04),
                    1.0,
                ))),
                border: iced::Border {
                    color: theme.text_color(0.10),
                    width: 0.8,
                    radius: Radius::new(10.0),
                },
                ..Default::default()
            })
            .padding(8)
            .width(Fill)
    }
}

impl PartialEq for ClipBoardContentType {
    /// Let cliboard items be comparable
    fn eq(&self, other: &Self) -> bool {
        if let Self::Text(a) = self
            && let Self::Text(b) = other
        {
            return a == b;
        } else if let Self::Image(image_data) = self
            && let Self::Image(other_image_data) = other
        {
            return image_data.bytes == other_image_data.bytes;
        }
        false
    }
}
