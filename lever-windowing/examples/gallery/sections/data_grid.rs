use super::section_card;
use crate::GalleryApp;
use crate::Message;
use lever_core::theme::Theme;
use lever_core::widget::Widget;
use lever_core::widgets::{
    BoxWidget, Button, DataGrid, DataGridColumn, Flex, Label, ProgressBar, Spacer,
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct MockUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub status: String,
    pub progress: f32,
}

pub fn view(app: &GalleryApp, theme: &Theme) -> Box<dyn Widget<Message>> {
    let mut users = Vec::with_capacity(10000);
    for i in 0..10000 {
        users.push(MockUser {
            id: i + 1,
            name: format!("User {}", i + 1),
            email: format!("user{}@example.com", i + 1),
            status: if i % 5 == 0 {
                "Active"
            } else if i % 3 == 0 {
                "Away"
            } else {
                "Offline"
            }
            .to_string(),
            progress: (i as f32 * 0.01) % 1.0,
        });
    }
    let users = Arc::new(users);

    let text_muted = theme.text_muted;
    let success_color = theme.success;
    let warning_color = theme.secondary;
    let surface_color = theme.surface;

    section_card(
        "Data & Tables",
        "Virtualized data grid handling 10,000 rows with sticky headers.",
        Box::new(lever_core::widgets::Expanded::new(Box::new(
            BoxWidget::new(surface_color)
                .with_radius(8.0)
                .with_child(Box::new(
                    DataGrid::new("demo-grid", users)
                        .with_column(DataGridColumn::new("ID", 80.0, move |u: &MockUser, _| {
                            Box::new(Label::new(format!("#{}", u.id)).with_color(text_muted))
                        }))
                        .with_column(DataGridColumn::new("Name", 150.0, |u: &MockUser, _| {
                            Box::new(Label::new(&u.name))
                        }))
                        .with_column(DataGridColumn::new(
                            "Email",
                            220.0,
                            move |u: &MockUser, _| {
                                Box::new(Label::new(&u.email).with_color(text_muted))
                            },
                        ))
                        .with_column(DataGridColumn::new(
                            "Status",
                            120.0,
                            move |u: &MockUser, _| {
                                let color = match u.status.as_str() {
                                    "Active" => success_color,
                                    "Away" => warning_color,
                                    _ => text_muted,
                                };
                                Box::new(
                                    Flex::row(vec![
                                        Box::new(
                                            BoxWidget::new(color)
                                                .with_radius(4.0)
                                                .with_size(8.0, 8.0),
                                        ),
                                        Box::new(Spacer::horizontal(8.0)),
                                        Box::new(
                                            Label::new(&u.status).with_color(color).with_size(12.0),
                                        ),
                                    ])
                                    .with_cross_axis_alignment(
                                        lever_core::layout::CrossAxisAlignment::Center,
                                    ),
                                )
                            },
                        ))
                        .with_column(DataGridColumn::new(
                            "Task Progress",
                            200.0,
                            |u: &MockUser, _| {
                                Box::new(
                                    Flex::column(vec![Box::new(ProgressBar::new(
                                        format!("p-{}", u.id),
                                        u.progress,
                                    ))])
                                    .with_main_axis_alignment(
                                        lever_core::MainAxisAlignment::Center,
                                    ),
                                )
                            },
                        ))
                        .with_column(DataGridColumn::new("Actions", 120.0, |u: &MockUser, _| {
                            let user_id = u.id;
                            Box::new(lever_core::widgets::Center::new(Box::new(
                                Button::new(format!("edit-{}", user_id), "Edit")
                                    .with_size(lever_core::widgets::ButtonSize::Small)
                                    .on_click(move || {
                                        Message::ButtonClicked(format!("Edit User {}", user_id))
                                    }),
                            )))
                        }))
                        .with_offset(app.grid_offset)
                        .on_scroll(|offset| Message::GridScrolled(offset)),
                )),
        ))),
        0,
        theme,
    )
}
