use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{Flex, Label, Spacer, TabItem, TabStyle, Tabs};
use lever_core::MainAxisAlignment;

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    let test_tex = app.test_image.unwrap_or(lever_core::types::TextureId(0));

    section_card(
        "Navigation & Tabs",
        "Modern, animated tab systems for switching views.",
        Box::new(Flex::column(vec![
            Box::new(
                Label::new("Underline Style (Default)")
                    .with_size(12.0)
                    .with_color(theme.text_muted),
            ),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            Box::new(
                Tabs::new(
                    "tabs-underline",
                    vec![
                        TabItem::new("Account"),
                        TabItem::new("Security"),
                        TabItem::new("Notifications"),
                        TabItem::new("Advanced").with_disabled(true),
                    ],
                    app.active_tab,
                )
                .on_change(|idx| Message::TabChanged(idx)),
            ),
            Box::new(Spacer::new().with_size(10.0, 32.0)),
            Box::new(
                Label::new("Pill Style with Icons")
                    .with_size(12.0)
                    .with_color(theme.text_muted),
            ),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            Box::new(
                Tabs::new(
                    "tabs-pill",
                    vec![
                        TabItem::new("Home").with_icon(test_tex),
                        TabItem::new("Search").with_icon(test_tex),
                        TabItem::new("Library").with_icon(test_tex),
                    ],
                    app.active_pill_tab,
                )
                .with_style(TabStyle::Pill)
                .on_change(|idx| Message::TabPillChanged(idx)),
            ),
            Box::new(Spacer::new().with_size(10.0, 32.0)),
            Box::new(
                Label::new("Full Width (Distributed)")
                    .with_size(12.0)
                    .with_color(theme.text_muted),
            ),
            Box::new(Spacer::new().with_size(10.0, 8.0)),
            Box::new(
                Tabs::new(
                    "tabs-full",
                    vec![
                        TabItem::new("Left"),
                        TabItem::new("Middle"),
                        TabItem::new("Right"),
                    ],
                    app.active_full_tab,
                )
                .with_full_width(true)
                .with_alignment(MainAxisAlignment::SpaceEvenly)
                .on_change(|idx| Message::TabFullChanged(idx)),
            ),
        ])),
        0,
        theme,
    )
}
