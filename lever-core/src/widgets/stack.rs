use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Alignment, Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

pub struct Stack<M> {
    pub children: Vec<Box<dyn Widget<M>>>,
    pub alignment: Alignment,
}

impl<M> Stack<M> {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            alignment: Alignment::TopLeft,
        }
    }

    pub fn with_child(mut self, child: Box<dyn Widget<M>>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget<M>>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl<M: 'static> Widget<M> for Stack<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut max_width = 0.0f32;
        let mut max_height = 0.0f32;

        for child in &self.children {
            let res = child.layout(
                Constraints::loose(constraints.max_width, constraints.max_height),
                &[],
                text_system,
                theme,
            );
            max_width = max_width.max(res.size.width);
            max_height = max_height.max(res.size.height);
        }

        LayoutResult {
            size: Size {
                width: max_width.clamp(constraints.min_width, constraints.max_width),
                height: max_height.clamp(constraints.min_height, constraints.max_height),
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
        for child in &self.children {
            let res = child.layout(
                Constraints::loose(rect.width, rect.height),
                &[],
                text_system,
                theme,
            );

            let (dx, dy) = self.alignment.align(res.size, rect.size());
            let child_rect = Rect {
                x: rect.x + dx,
                y: rect.y + dy,
                width: res.size.width,
                height: res.size.height,
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

        for child in self.children.iter_mut().rev() {
            let res = child.layout(
                Constraints::loose(rect.width, rect.height),
                &[],
                text_system,
                theme,
            );

            let (dx, dy) = self.alignment.align(res.size, rect.size());
            let child_rect = Rect {
                x: rect.x + dx,
                y: rect.y + dy,
                width: res.size.width,
                height: res.size.height,
            };

            messages.extend(child.on_event(
                event,
                child_rect,
                text_system,
                theme,
                focused_id,
                consumed,
            ));

            if *consumed {
                break;
            }
        }

        messages
    }
}
