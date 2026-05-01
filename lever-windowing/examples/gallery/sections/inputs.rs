use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{Flex, Spacer, TextInput};

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    let test_tex = app.test_image.unwrap_or(lever_core::types::TextureId(0));

    section_card(
        "Form Inputs",
        "Modern text fields with validation, icons, and password support.",
        Box::new(
            Flex::column(vec![
                Box::new(
                    TextInput::new("input-1")
                        .with_placeholder("Username or email")
                        .with_text(app.text_input1.clone())
                        .with_cursor(app.cursor1)
                        .on_input(|text, idx| Message::TextInput1Changed(text, idx)),
                ),
                Box::new(Spacer::new().with_size(10.0, 16.0)),
                Box::new(
                    TextInput::new("input-2")
                        .with_placeholder("Password")
                        .with_text(app.text_input2.clone())
                        .with_cursor(app.cursor2)
                        .with_password(true)
                        .with_leading_icon(test_tex)
                        .on_input(|text, idx| Message::TextInput2Changed(text, idx)),
                ),
                Box::new(Spacer::new().with_size(10.0, 32.0)),
                Box::new(
                    TextInput::new("input-3")
                        .with_placeholder("Search something...")
                        .with_text(app.text_input3.clone())
                        .with_cursor(app.cursor3)
                        .with_trailing_icon(test_tex)
                        .on_input(|text, idx| Message::TextInput3Changed(text, idx)),
                ),
                Box::new(Spacer::new().with_size(10.0, 16.0)),
                Box::new(
                    TextInput::new("input-4")
                        .with_placeholder("Invalid input")
                        .with_text(app.text_input4.clone())
                        .with_cursor(app.cursor4)
                        .with_error("This field is required")
                        .on_input(|text, idx| Message::TextInput4Changed(text, idx)),
                ),
            ])
            .with_cross_axis_alignment(lever_core::layout::CrossAxisAlignment::Stretch),
        ),
        0,
        theme,
    )
}
