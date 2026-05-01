use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Color, Rect, Size};
use crate::widget::Widget;

pub struct Overlay<M> {
    pub color: Color,
    pub child: Option<Box<dyn Widget<M>>>,
    pub alignment: crate::layout::Alignment,
    pub on_dismiss: Option<Box<dyn Fn() -> M>>,
}

impl<M> Overlay<M> {
    pub fn new() -> Self {
        Self {
            color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            child: None,
            alignment: crate::layout::Alignment::Center,
            on_dismiss: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_child(mut self, child: impl Widget<M> + 'static) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn with_alignment(mut self, alignment: crate::layout::Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn on_dismiss<F: Fn() -> M + 'static>(mut self, f: F) -> Self {
        self.on_dismiss = Some(Box::new(f));
        self
    }
}

impl<M: 'static> Widget<M> for Overlay<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        if let Some(child) = &self.child {
            child.layout(
                Constraints::loose(constraints.max_width, constraints.max_height),
                &[],
                text_system,
                theme,
            );
        }

        LayoutResult {
            size: Size {
                width: constraints.max_width,
                height: constraints.max_height,
            },
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<crate::types::Point>,
    ) {
        draw_list.colored_rect(rect, self.color, 0.0);

        if let Some(child) = &self.child {
            let child_res = child.layout(
                Constraints::loose(rect.width, rect.height),
                &[],
                text_system,
                theme,
            );

            let (ox, oy) = self.alignment.align(child_res.size, rect.size());
            let child_rect = Rect {
                x: rect.x + ox,
                y: rect.y + oy,
                width: child_res.size.width,
                height: child_res.size.height,
            };

            child.draw(
                child_rect,
                draw_list,
                text_system,
                theme,
                focused_id,
                pointer_pos,
            );
        }
    }

    fn on_event(
        &mut self,
        event: &FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let mut child_rect = None;

        if let Some(child) = &mut self.child {
            let child_res = child.layout(
                Constraints::loose(rect.width, rect.height),
                &[],
                text_system,
                theme,
            );

            let (ox, oy) = self.alignment.align(child_res.size, rect.size());
            let crect = Rect {
                x: rect.x + ox,
                y: rect.y + oy,
                width: child_res.size.width,
                height: child_res.size.height,
            };
            child_rect = Some(crect);

            messages.extend(child.on_event(event, crect, text_system, theme, focused_id, consumed));
        }

        if !*consumed {
            match event {
                FrameworkEvent::PointerDown { position, .. } => {
                    if rect.contains(*position) {
                        let hit_child = child_rect.map_or(false, |r| r.contains(*position));
                        if !hit_child {
                            if let Some(on_dismiss) = &self.on_dismiss {
                                messages.push(on_dismiss());
                                *consumed = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if !*consumed {
            if let Some(pos) = event.pointer_pos() {
                if rect.contains(pos) {
                    *consumed = true;
                }
            }
        }

        messages
    }
}
