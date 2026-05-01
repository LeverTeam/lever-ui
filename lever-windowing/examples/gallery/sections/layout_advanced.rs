use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{BoxWidget, Button, Flex, Label, SideBar, SplitAxis, SplitView};

pub fn view(app: &crate::GalleryApp, theme: &Theme) -> Box<dyn Widget<crate::Message>> {
    let sidebar_content = Box::new(
        BoxWidget::new(theme.surface).with_child(Box::new(
            BoxWidget::transparent()
                .with_padding(lever_core::types::SideOffsets::all(20.0))
                .with_child(Box::new(
                    Flex::column(vec![
                        Box::new(Label::new("Navigation").with_size(18.0)),
                        Box::new(Label::new("Overview").with_color(theme.text_muted)),
                        Box::new(Label::new("Settings").with_color(theme.text_muted)),
                        Box::new(Label::new("Profile").with_color(theme.text_muted)),
                        Box::new(Label::new("Analytics").with_color(theme.text_muted)),
                    ])
                    .with_gap(12.0),
                )),
        )),
    );

    let split_content_first = Box::new(
        BoxWidget::new(theme.surface_variant)
            .with_radius(8.0)
            .with_child(Box::new(lever_core::widgets::Center::new(Box::new(
                Label::new("Resizable Left Pane").with_size(14.0),
            )))),
    );

    let split_content_second = Box::new(BoxWidget::new(theme.surface).with_radius(8.0).with_child(
        Box::new(lever_core::widgets::Center::new(Box::new(
            Label::new("Resizable Right Pane").with_size(14.0),
        ))),
    ));

    let split_view = Box::new(
        SplitView::new(
            "example-split",
            SplitAxis::Horizontal,
            split_content_first,
            split_content_second,
        )
        .with_ratio(0.4),
    );

    let is_collapsed = app.is_sidebar_collapsed;

    Box::new(Flex::column(vec![
        Box::new(Label::new("Advanced Layouts").with_size(28.0)),
        Box::new(Label::new("SplitView and SideBar containers with state persistence.").with_color(theme.text_muted)),

        Box::new(Flex::row(vec![
            Box::new(Button::new("toggle-sidebar-btn", "Toggle Sidebar Visibility")
                .on_click(move || Message::ToggleSidebar(!is_collapsed))),
        ])),

        Box::new(BoxWidget::new(theme.border.with_alpha(0.1))
            .with_radius(12.0)
            .with_height(600.0)
            .with_child(Box::new(SideBar::new(
                "example-sidebar",
                sidebar_content,
                split_view,
            ).with_collapsed(app.is_sidebar_collapsed)))),

        Box::new(Label::new("The divider position above is persisted in memory. Try resizing and switching tabs.").with_size(12.0).with_color(theme.text_muted)),
    ]).with_gap(24.0))
}
