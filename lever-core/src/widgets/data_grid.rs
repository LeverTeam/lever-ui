use crate::draw::DrawList;
use crate::event::FrameworkEvent;
use crate::layout::{Constraints, LayoutNode, LayoutResult};
use crate::types::{Point, Rect, Size};
use crate::widget::Widget;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DataGridState {
    pub scroll_offset: Point,
}

impl Default for DataGridState {
    fn default() -> Self {
        Self {
            scroll_offset: Point { x: 0.0, y: 0.0 },
        }
    }
}

pub struct DataGridColumn<M, T> {
    pub title: String,
    pub width: f32,
    pub render: Arc<dyn Fn(&T, usize) -> Box<dyn Widget<M>> + Send + Sync>,
}

impl<M, T> DataGridColumn<M, T> {
    pub fn new<F>(title: impl Into<String>, width: f32, render: F) -> Self
    where
        F: Fn(&T, usize) -> Box<dyn Widget<M>> + Send + Sync + 'static,
    {
        Self {
            title: title.into(),
            width,
            render: Arc::new(render),
        }
    }
}

pub struct DataGrid<M, T> {
    pub id: String,
    pub columns: Vec<DataGridColumn<M, T>>,
    pub data: Arc<Vec<T>>,
    pub row_height: f32,
    pub header_height: f32,
    pub scroll_offset: Option<Point>,
    pub on_scroll: Option<Box<dyn Fn(Point) -> M>>,
}

impl<M, T> DataGrid<M, T> {
    pub fn new(id: impl Into<String>, data: Arc<Vec<T>>) -> Self {
        Self {
            id: id.into(),
            columns: Vec::new(),
            data,
            row_height: 40.0,
            header_height: 44.0,
            scroll_offset: None,
            on_scroll: None,
        }
    }

    pub fn with_offset(mut self, offset: Point) -> Self {
        self.scroll_offset = Some(offset);
        self
    }

    pub fn with_column(mut self, column: DataGridColumn<M, T>) -> Self {
        self.columns.push(column);
        self
    }

    pub fn with_row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }

    pub fn on_scroll<F>(mut self, callback: F) -> Self
    where
        F: Fn(Point) -> M + 'static,
    {
        self.on_scroll = Some(Box::new(callback));
        self
    }

    fn total_content_height(&self) -> f32 {
        self.header_height + (self.data.len() as f32 * self.row_height)
    }

    fn total_content_width(&self) -> f32 {
        self.columns.iter().map(|c| c.width).sum()
    }
}

impl<M: 'static, T: 'static> Widget<M> for DataGrid<M, T> {
    fn layout(
        &self,
        constraints: Constraints,
        _children: &[LayoutNode],
        _text_system: &mut crate::text::TextSystem,
        _theme: &crate::theme::Theme,
    ) -> LayoutResult {
        let height = if constraints.max_height.is_finite() {
            constraints.max_height
        } else {
            constraints.min_height
        };

        let width = if constraints.max_width.is_finite() {
            constraints.max_width
        } else {
            self.total_content_width()
        };

        LayoutResult {
            size: constraints.clamp_size(Size { width, height }),
        }
    }

    fn draw(
        &self,
        rect: Rect,
        draw_list: &mut DrawList,
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
        focused_id: Option<&str>,
        pointer_pos: Option<Point>,
    ) {
        let state = crate::state::get_state::<DataGridState>(&self.id).unwrap_or_default();
        let scroll_offset = self.scroll_offset.unwrap_or(state.scroll_offset);

        draw_list.clip_push(rect);

        draw_list.colored_rect(rect, theme.surface, 0.0);

        let header_rect = Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width,
            height: self.header_height,
        };

        draw_list.colored_rect(header_rect, theme.surface_variant, 0.0);
        draw_list.line(
            Point::new(rect.x, rect.y + self.header_height),
            Point::new(rect.x + rect.width, rect.y + self.header_height),
            1.0,
            theme.border,
        );

        let mut x_offset = 0.0;
        for col in &self.columns {
            let col_header_rect = Rect {
                x: header_rect.x + x_offset - scroll_offset.x,
                y: header_rect.y,
                width: col.width,
                height: self.header_height,
            };

            if col_header_rect.x + col_header_rect.width > rect.x
                && col_header_rect.x < rect.x + rect.width
            {
                let label: crate::widgets::Label<M> = crate::widgets::Label::new(&col.title)
                    .with_size(12.0)
                    .with_color(theme.text_muted);

                let (_lx, ly) = crate::layout::Alignment::CenterLeft
                    .align(Size::new(col.width - 24.0, 14.0), col_header_rect.size());

                label.draw(
                    Rect {
                        x: col_header_rect.x + 12.0,
                        y: col_header_rect.y + ly,
                        width: col.width - 24.0,
                        height: 14.0,
                    },
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );
            }
            x_offset += col.width;
        }

        let body_rect = Rect {
            x: rect.x,
            y: rect.y + self.header_height,
            width: rect.width,
            height: rect.height - self.header_height,
        };

        draw_list.clip_push(body_rect);

        let start_index = (scroll_offset.y / self.row_height).floor() as usize;
        let end_index = ((scroll_offset.y + body_rect.height) / self.row_height).ceil() as usize;
        let end_index = end_index.min(self.data.len());

        for i in start_index..end_index {
            let row_y = body_rect.y + (i as f32 * self.row_height) - scroll_offset.y;

            if i % 2 == 1 {
                draw_list.colored_rect(
                    Rect {
                        x: rect.x,
                        y: row_y,
                        width: rect.width,
                        height: self.row_height,
                    },
                    theme.text.with_alpha(0.02),
                    0.0,
                );
            }

            let mut x_offset = 0.0;
            for col in &self.columns {
                let cell_rect = Rect {
                    x: body_rect.x + x_offset - scroll_offset.x,
                    y: row_y,
                    width: col.width,
                    height: self.row_height,
                };

                let cell_widget = (col.render)(&self.data[i], i);
                let cell_inner_rect = cell_rect.inset(12.0, 0.0);
                cell_widget.draw(
                    cell_inner_rect,
                    draw_list,
                    text_system,
                    theme,
                    focused_id,
                    pointer_pos,
                );
                x_offset += col.width;
            }

            draw_list.line(
                Point::new(rect.x, row_y + self.row_height),
                Point::new(rect.x + rect.width, row_y + self.row_height),
                1.0,
                theme.border.with_alpha(0.3),
            );
        }

        draw_list.clip_pop();
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
        let mut state = crate::state::get_state::<DataGridState>(&self.id).unwrap_or_default();
        let mut scroll_offset = self.scroll_offset.unwrap_or(state.scroll_offset);

        if let FrameworkEvent::Scroll { position, delta } = event {
            if rect.contains(*position) {
                let max_scroll_y = (self.total_content_height() - rect.height).max(0.0);
                let max_scroll_x = (self.total_content_width() - rect.width).max(0.0);

                scroll_offset.y = (scroll_offset.y + delta.y).clamp(0.0, max_scroll_y);
                scroll_offset.x = (scroll_offset.x + delta.x).clamp(0.0, max_scroll_x);

                state.scroll_offset = scroll_offset;
                crate::state::set_state(&self.id, state);

                if let Some(on_scroll) = &self.on_scroll {
                    messages.push(on_scroll(scroll_offset));
                }

                *consumed = true;
                return messages;
            }
        }

        let body_rect = Rect {
            x: rect.x,
            y: rect.y + self.header_height,
            width: rect.width,
            height: rect.height - self.header_height,
        };

        if body_rect.contains(event.pointer_pos().unwrap_or(Point::new(-1000.0, -1000.0))) {
            let start_index = (scroll_offset.y / self.row_height).floor() as usize;
            let end_index =
                ((scroll_offset.y + body_rect.height) / self.row_height).ceil() as usize;
            let end_index = end_index.min(self.data.len());

            for i in start_index..end_index {
                let row_y = body_rect.y + (i as f32 * self.row_height) - scroll_offset.y;
                let mut x_offset = 0.0;

                for col in &self.columns {
                    let cell_rect = Rect {
                        x: body_rect.x + x_offset - scroll_offset.x,
                        y: row_y,
                        width: col.width,
                        height: self.row_height,
                    };

                    if cell_rect
                        .contains(event.pointer_pos().unwrap_or(Point::new(-1000.0, -1000.0)))
                    {
                        let mut cell_widget = (col.render)(&self.data[i], i);
                        let cell_inner_rect = cell_rect.inset(12.0, 0.0);

                        messages.extend(cell_widget.on_event(
                            event,
                            cell_inner_rect,
                            text_system,
                            theme,
                            focused_id,
                            consumed,
                        ));

                        if *consumed {
                            return messages;
                        }
                    }
                    x_offset += col.width;
                }
            }
        }

        messages
    }
}
