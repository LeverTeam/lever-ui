use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;

pub mod animation;
pub mod controls;
pub mod data_grid;
pub mod inputs;
pub mod layout;
pub mod layout_advanced;
pub mod media;
pub mod navigation;
pub mod progress;
pub mod typography;

pub fn section_card(
    title: &str,
    subtitle: &str,
    child: Box<dyn Widget<Message>>,
    flex: u32,
    theme: &Theme,
) -> Box<dyn Widget<Message>> {
    use lever_core::layout::CrossAxisAlignment;
    use lever_core::types::SideOffsets;
    use lever_core::widgets::{BoxWidget, Flex, Label, Spacer};

    Box::new(
        BoxWidget::new(theme.surface)
            .with_radius(12.0)
            .with_padding(SideOffsets::all(24.0))
            .with_flex(flex)
            .with_child(Box::new(
                Flex::column(vec![
                    Box::new(
                        Flex::column(vec![
                            Box::new(Label::new(title).with_size(24.0)),
                            Box::new(Spacer::new().with_size(10.0, 6.0)),
                            Box::new(Label::new(subtitle).with_color(theme.text_muted)),
                        ])
                        .with_cross_axis_alignment(CrossAxisAlignment::Stretch),
                    ),
                    Box::new(Spacer::new().with_size(10.0, 32.0)),
                    child,
                ])
                .with_cross_axis_alignment(CrossAxisAlignment::Stretch),
            )),
    ) as Box<dyn Widget<Message>>
}
