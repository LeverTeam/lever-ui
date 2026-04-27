use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect, Size};
use crate::widget::Widget;

pub struct Scroll<M> {
    pub child: Box<dyn Widget<M>>,
    pub scroll_offset: Point,
    pub on_scroll: Option<Box<dyn Fn(Point) -> M>>,
    pub flex: u32,
}

impl<M> Scroll<M> {
    pub fn new(child: Box<dyn Widget<M>>) -> Self {
        Self {
            child,
            scroll_offset: Point { x: 0.0, y: 0.0 },
            on_scroll: None,
            flex: 0,
        }
    }

    pub fn with_offset(mut self, offset: Point) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self
    where
        F: Fn(Point) -> M + 'static,
    {
        self.on_scroll = Some(Box::new(callback));
        self
    }

    pub fn with_flex(mut self, flex: u32) -> Self {
        self.flex = flex;
        self
    }
}

impl<M: 'static> Widget<M> for Scroll<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let child_constraints = Constraints {
            min_width: constraints.min_width,
            max_width: constraints.max_width,
            min_height: 0.0,
            max_height: f32::INFINITY,
        };

        self.child
            .layout(child_constraints, &[], text_system, theme);

        LayoutResult {
            size: constraints.clamp_size(Size {
                width: constraints.max_width,
                height: constraints.max_height,
            }),
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
        draw_list.clip_push(rect);

        let child_rect = Rect {
            x: rect.x - self.scroll_offset.x,
            y: rect.y - self.scroll_offset.y,
            width: rect.width, // Constrain width to viewport for vertical scrolling
            height: f32::INFINITY,
        };

        self.child.draw(
            child_rect,
            draw_list,
            text_system,
            theme,
            focused_id,
            pointer_pos,
        );

        draw_list.clip_pop();
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

        // Handle scroll wheel
        if let FrameworkEvent::Scroll { position, delta } = event {
            if rect.contains(*position) {
                self.scroll_offset.x = (self.scroll_offset.x + delta.x).max(0.0);
                self.scroll_offset.y = (self.scroll_offset.y + delta.y).max(0.0);
                if let Some(on_scroll) = &self.on_scroll {
                    messages.push(on_scroll(self.scroll_offset));
                }
                *consumed = true;
                return messages;
            }
        }

        let child_rect = Rect {
            x: rect.x - self.scroll_offset.x,
            y: rect.y - self.scroll_offset.y,
            width: rect.width,
            height: f32::INFINITY,
        };

        if let Some(pos) = event.pointer_pos() {
            if rect.contains(pos) {
                messages.extend(self.child.on_event(
                    event,
                    child_rect,
                    text_system,
                    theme,
                    focused_id,
                    consumed,
                ));

                if *consumed {
                    return messages;
                }
            }
        } else {
            messages.extend(self.child.on_event(
                event,
                child_rect,
                text_system,
                theme,
                focused_id,
                consumed,
            ));

            if *consumed {
                return messages;
            }
        }

        messages
    }

    fn flex(&self) -> u32 {
        self.flex
    }
}
