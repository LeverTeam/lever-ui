use crate::draw::{DrawCommand, DrawList};
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::state::{get_or_set_state, update_state};
use crate::types::{Color, Point, Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Default)]
struct DropdownState {
    is_open: bool,
}

pub struct Dropdown<M> {
    pub id: String,
    pub items: Vec<String>,
    pub selected_index: Option<usize>,
    pub on_select: Option<Box<dyn Fn(usize) -> M>>,
}

impl<M> Dropdown<M> {
    pub fn new(id: impl Into<String>, items: Vec<String>, selected_index: Option<usize>) -> Self {
        Self {
            id: id.into(),
            items,
            selected_index,
            on_select: None,
        }
    }

    pub fn on_select<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.on_select = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Dropdown<M> {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut max_width: f32 = 120.0;
        for item in &self.items {
            let layout = text_system.shape(item, 14.0, theme.text);
            max_width = max_width.max(layout.width + 48.0);
        }

        let size = constraints.clamp_size(Size {
            width: max_width,
            height: 36.0,
        });
        LayoutResult { size }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        let state = get_or_set_state::<DropdownState, _>(&self.id, || DropdownState::default());
        let is_hovered = pointer_pos.map_or(false, |pos| rect.contains(pos));

        // Draw the button
        let bg_color = if state.is_open {
            theme.surface_variant
        } else if is_hovered {
            theme.surface_variant
        } else {
            theme.surface
        };
        draw_list.rounded_rect(rect, bg_color, theme.radius_md);
        draw_list.stroke_rect(rect, theme.border, theme.radius_md, 1.0);

        let label = self
            .selected_index
            .and_then(|i| self.items.get(i))
            .map(|s| s.as_str())
            .unwrap_or("Select...");

        let layout = text_system.shape(label, 14.0, theme.text);
        draw_list.text(
            Point {
                x: (rect.x + 12.0).round(),
                y: (rect.y + (rect.height - layout.height) / 2.0).round(),
            },
            layout.glyphs,
        );

        // Simple triangle for chevron
        let chevron_color = theme.text_muted;
        let chevron_size = 8.0;
        let chevron_height = 5.0;
        let chevron_x = rect.x + rect.width - 24.0;
        let chevron_y = rect.y + (rect.height - chevron_height) / 2.0;

        draw_list.triangle(
            Point {
                x: chevron_x,
                y: chevron_y,
            },
            Point {
                x: chevron_x + chevron_size,
                y: chevron_y,
            },
            Point {
                x: chevron_x + chevron_size / 2.0,
                y: chevron_y + chevron_height,
            },
            chevron_color,
        );

        // DEFERRED: Draw Menu
        if state.is_open {
            let menu_width = rect.width;
            let item_height = 36.0;
            let menu_height = self.items.len() as f32 * item_height + 8.0;

            let menu_rect = Rect {
                x: rect.x,
                y: rect.y + rect.height + 4.0,
                width: menu_width,
                height: menu_height,
            };

            // Shadow and Background
            draw_list.push_deferred(DrawCommand::RoundedRect {
                rect: menu_rect,
                color: theme.surface,
                radius: theme.radius_md,
                shadow: Some(crate::types::BoxShadow {
                    offset: Point { x: 0.0, y: 10.0 },
                    blur: 30.0,
                    color: Color::rgba(0.0, 0.0, 0.0, 0.3),
                }),
            });
            draw_list.push_deferred(DrawCommand::Stroke {
                rect: menu_rect,
                color: theme.border,
                radius: theme.radius_md,
                thickness: 1.0,
            });

            for (i, item) in self.items.iter().enumerate() {
                let item_rect = Rect {
                    x: menu_rect.x + 4.0,
                    y: menu_rect.y + 4.0 + (i as f32 * item_height),
                    width: menu_width - 8.0,
                    height: item_height,
                };

                let item_hovered = pointer_pos.map_or(false, |pos| item_rect.contains(pos));
                if item_hovered {
                    draw_list.push_deferred(DrawCommand::RoundedRect {
                        rect: item_rect,
                        color: theme.surface_variant,
                        radius: theme.radius_sm,
                        shadow: None,
                    });
                }

                let text_layout = text_system.shape(item, 14.0, theme.text);
                draw_list.push_deferred(DrawCommand::Text {
                    pos: Point {
                        x: (item_rect.x + 8.0).round(),
                        y: (item_rect.y + (item_height - text_layout.height) / 2.0).round(),
                    },
                    glyphs: text_layout.glyphs,
                });
            }
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let state = get_or_set_state::<DropdownState, _>(&self.id, || DropdownState::default());

        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary {
                    if rect.contains(*position) {
                        *focused_id = Some(self.id.clone());
                        let next_open = !state.is_open;
                        update_state::<DropdownState, _>(&self.id, |s| s.is_open = next_open);
                    } else if state.is_open {
                        // Check menu items
                        let menu_width = rect.width;
                        let item_height = 36.0;
                        let menu_y = rect.y + rect.height + 4.0;

                        for i in 0..self.items.len() {
                            let item_rect = Rect {
                                x: rect.x + 4.0,
                                y: menu_y + 4.0 + (i as f32 * item_height),
                                width: menu_width - 8.0,
                                height: item_height,
                            };

                            if item_rect.contains(*position) {
                                if let Some(on_select) = &self.on_select {
                                    messages.push(on_select(i));
                                }
                                update_state::<DropdownState, _>(&self.id, |s| s.is_open = false);
                                return messages;
                            }
                        }

                        // Clicked outside, close menu
                        update_state::<DropdownState, _>(&self.id, |s| s.is_open = false);
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
