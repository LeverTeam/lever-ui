use crate::animated::{animated_color, animated_spring};
use crate::animation::Spring;
use crate::draw::DrawList;
use crate::event::{FrameworkEvent, PointerButton};
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect, Size};
use crate::widget::Widget;

pub struct Tabs<M> {
    pub id: String,
    pub items: Vec<String>,
    pub active_index: usize,
    pub on_change: Option<Box<dyn Fn(usize) -> M>>,
}

impl<M> Tabs<M> {
    pub fn new(id: impl Into<String>, items: Vec<String>, active_index: usize) -> Self {
        Self {
            id: id.into(),
            items,
            active_index,
            on_change: None,
        }
    }

    pub fn on_change<F>(mut self, f: F) -> Self
    where
        F: Fn(usize) -> M + 'static,
    {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Tabs<M> {
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
        let mut total_width = 0.0;
        for item in &self.items {
            let layout = text_system.shape(item, 14.0, theme.text, None);
            total_width += layout.width + 32.0; // Padding
        }

        let size = constraints.clamp_size(Size {
            width: total_width,
            height: 40.0,
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
        _pointer_pos: Option<crate::types::Point>,
    ) {
        let mut current_x = rect.x;
        let mut tab_rects = Vec::new();

        // Draw items and collect their rects for indicator animation
        for (i, item) in self.items.iter().enumerate() {
            let layout = text_system.shape(item, 14.0, theme.text, None);
            let tab_width = layout.width + 32.0;
            let tab_rect = Rect {
                x: current_x,
                y: rect.y,
                width: tab_width,
                height: rect.height,
            };
            tab_rects.push(tab_rect);

            let is_active = i == self.active_index;
            let target_color = if is_active {
                theme.primary
            } else {
                theme.text_muted
            };
            let text_color =
                animated_color(&format!("{}_tab_text_{}", self.id, i), target_color, 0.2);

            draw_list.text(
                Point {
                    x: (current_x + 16.0).round(),
                    y: (rect.y + (rect.height - layout.height) / 2.0).round(),
                },
                text_system.shape(item, 14.0, text_color, None).glyphs,
            );

            current_x += tab_width;
        }

        // Animate indicator
        if let Some(active_rect) = tab_rects.get(self.active_index) {
            let indicator_x = animated_spring(
                &format!("{}_indicator_x", self.id),
                active_rect.x,
                Spring::SNAPPY,
            );
            let indicator_width = animated_spring(
                &format!("{}_indicator_w", self.id),
                active_rect.width,
                Spring::SNAPPY,
            );

            let indicator_rect = Rect {
                x: indicator_x,
                y: rect.y + rect.height - 3.0,
                width: indicator_width,
                height: 3.0,
            };

            draw_list.rounded_rect(indicator_rect, theme.primary, 1.5);
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        _focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        match event {
            FrameworkEvent::PointerDown { position, button } => {
                if *button == PointerButton::Primary && rect.contains(*position) {
                    let mut current_x = rect.x;
                    for (i, item) in self.items.iter().enumerate() {
                        let layout = text_system.shape(item, 14.0, theme.text, None);
                        let tab_width = layout.width + 32.0;
                        let tab_rect = Rect {
                            x: current_x,
                            y: rect.y,
                            width: tab_width,
                            height: rect.height,
                        };

                        if tab_rect.contains(*position) {
                            if i != self.active_index {
                                if let Some(on_change) = &self.on_change {
                                    messages.push(on_change(i));
                                }
                            }
                            break;
                        }
                        current_x += tab_width;
                    }
                }
            }
            _ => {}
        }
        messages
    }
}
