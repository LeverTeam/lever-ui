use crate::draw::DrawList;
use crate::layout::{Constraints, GridLayout, GridTrack, LayoutNode, LayoutResult};
use crate::types::Rect;
use crate::widget::Widget;

pub struct Grid<M> {
    pub columns: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub children: Vec<Box<dyn Widget<M>>>,
    pub column_gap: f32,
    pub row_gap: f32,
}

impl<M> Default for Grid<M> {
    fn default() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            children: Vec::new(),
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }
}

impl<M> Grid<M> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_column(mut self, track: GridTrack) -> Self {
        self.columns.push(track);
        self
    }

    pub fn with_columns(mut self, tracks: Vec<GridTrack>) -> Self {
        self.columns.extend(tracks);
        self
    }

    pub fn with_row(mut self, track: GridTrack) -> Self {
        self.rows.push(track);
        self
    }

    pub fn with_rows(mut self, tracks: Vec<GridTrack>) -> Self {
        self.rows.extend(tracks);
        self
    }

    pub fn with_child(mut self, child: Box<dyn Widget<M>>) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_children(mut self, children: Vec<Box<dyn Widget<M>>>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn with_column_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self
    }

    pub fn with_row_gap(mut self, gap: f32) -> Self {
        self.row_gap = gap;
        self
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.column_gap = gap;
        self.row_gap = gap;
        self
    }
}

impl<M: 'static> Widget<M> for Grid<M> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let solver = GridLayout {
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            column_gap: self.column_gap,
            row_gap: self.row_gap,
        };
        let (result, _) = solver.layout(constraints, &self.children, text_system, theme);
        result
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
        let solver = GridLayout {
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            column_gap: self.column_gap,
            row_gap: self.row_gap,
        };
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
            theme,
        );

        for (i, child) in self.children.iter().enumerate() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;
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
        event: &crate::event::FrameworkEvent,
        rect: Rect,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: &mut Option<String>,
        consumed: &mut bool,
    ) -> Vec<M> {
        let mut messages = Vec::new();
        let solver = GridLayout {
            columns: self.columns.clone(),
            rows: self.rows.clone(),
            column_gap: self.column_gap,
            row_gap: self.row_gap,
        };
        let (_result, child_rects) = solver.layout(
            Constraints::tight(rect.width, rect.height),
            &self.children,
            text_system,
            theme,
        );

        for (i, child) in self.children.iter_mut().enumerate().rev() {
            let mut child_rect = child_rects[i];
            child_rect.x += rect.x;
            child_rect.y += rect.y;
            messages.extend(child.on_event(
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
}
