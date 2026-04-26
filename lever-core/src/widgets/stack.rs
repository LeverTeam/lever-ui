use crate::draw::DrawList;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

pub struct Stack<M> {
    pub id: Option<String>,
    pub children: Vec<Box<dyn Widget<M>>>,
}

impl<M> Stack<M> {
    pub fn new(children: Vec<Box<dyn Widget<M>>>) -> Self {
        Self { id: None, children }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<M: 'static> Widget<M> for Stack<M> {
    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let mut max_width: f32 = 0.0;
        let mut max_height: f32 = 0.0;

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
                width: max_width,
                height: max_height,
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
            child.draw(rect, draw_list, text_system, theme, focused_id, pointer_pos);
        }
    }

    fn on_event(
        &mut self,
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        for child in self.children.iter_mut().rev() {
            let res = child.on_event(event, rect, text_system, theme, focused_id);
            if !res.is_empty() {
                messages.extend(res);
                return messages;
            }
        }
        messages
    }
}


