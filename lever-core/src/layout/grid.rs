use crate::layout::{Constraints, LayoutResult};
use crate::types::{Rect, Size};
use crate::widget::Widget;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridTrack {
    Fixed(f32),
    Flex(u32),
    Auto,
}

pub struct GridLayout {
    pub columns: Vec<GridTrack>,
    pub rows: Vec<GridTrack>,
    pub column_gap: f32,
    pub row_gap: f32,
}

impl GridLayout {
    pub fn new(columns: Vec<GridTrack>, rows: Vec<GridTrack>) -> Self {
        Self {
            columns,
            rows,
            column_gap: 0.0,
            row_gap: 0.0,
        }
    }

    pub fn layout<M>(
        &self,
        constraints: Constraints,
        children: &[Box<dyn Widget<M>>],
        text_system: &mut crate::text::TextSystem,
        theme: &crate::theme::Theme,
    ) -> (LayoutResult, Vec<Rect>) {
        let col_count = self.columns.len().max(1);
        let row_count = if self.rows.is_empty() {
            (children.len() + col_count - 1) / col_count
        } else {
            self.rows.len()
        };

        let mut col_widths = vec![0.0; col_count];
        let mut row_heights = vec![0.0; row_count];

        let mut total_flex_cols = 0;
        let mut used_width = self.column_gap * (col_count as f32 - 1.0).max(0.0);

        for (i, track) in self.columns.iter().enumerate() {
            match track {
                GridTrack::Fixed(w) => {
                    col_widths[i] = *w;
                    used_width += w;
                }
                GridTrack::Flex(f) => {
                    total_flex_cols += f;
                }
                GridTrack::Auto => {}
            }
        }

        let mut total_flex_rows = 0;
        let mut used_height = self.row_gap * (row_count as f32 - 1.0).max(0.0);
        for (i, track) in self.rows.iter().enumerate() {
            match track {
                GridTrack::Fixed(h) => {
                    row_heights[i] = *h;
                    used_height += h;
                }
                GridTrack::Flex(f) => {
                    total_flex_rows += f;
                }
                GridTrack::Auto => {}
            }
        }

        for (i, child) in children.iter().enumerate() {
            let col = i % col_count;
            let row = i / col_count;
            if row >= row_count {
                break;
            }

            let col_track = self.columns.get(col).unwrap_or(&GridTrack::Auto);
            let row_track = self.rows.get(row).unwrap_or(&GridTrack::Auto);

            if matches!(col_track, GridTrack::Auto) || matches!(row_track, GridTrack::Auto) {
                let res = child.layout(
                    Constraints::loose(constraints.max_width, constraints.max_height),
                    &[],
                    text_system,
                    theme,
                );
                if matches!(col_track, GridTrack::Auto) {
                    if res.size.width > col_widths[col] {
                        used_width += res.size.width - col_widths[col];
                        col_widths[col] = res.size.width;
                    }
                }
                if matches!(row_track, GridTrack::Auto) {
                    if res.size.height > row_heights[row] {
                        used_height += res.size.height - row_heights[row];
                        row_heights[row] = res.size.height;
                    }
                }
            }
        }

        if total_flex_cols > 0 && constraints.max_width.is_finite() {
            let remaining_width = (constraints.max_width - used_width).max(0.0);
            let width_per_flex = remaining_width / total_flex_cols as f32;
            for (i, track) in self.columns.iter().enumerate() {
                if let GridTrack::Flex(f) = track {
                    col_widths[i] = width_per_flex * (*f as f32);
                    used_width += col_widths[i];
                }
            }
        }

        if total_flex_rows > 0 && constraints.max_height.is_finite() {
            let remaining_height = (constraints.max_height - used_height).max(0.0);
            let height_per_flex = remaining_height / total_flex_rows as f32;
            for (i, track) in self.rows.iter().enumerate() {
                if let GridTrack::Flex(f) = track {
                    row_heights[i] = height_per_flex * (*f as f32);
                    used_height += row_heights[i];
                }
            }
        }

        let mut final_width = used_width;
        let mut final_height = used_height;

        if final_width.is_nan() || final_width.is_infinite() {
            final_width = used_width.max(0.0);
            if final_width.is_infinite() {
                final_width = 0.0;
            }
        }
        if final_height.is_nan() || final_height.is_infinite() {
            final_height = used_height.max(0.0);
            if final_height.is_infinite() {
                final_height = 0.0;
            }
        }

        let final_size = constraints.clamp_size(Size {
            width: final_width,
            height: final_height,
        });

        let mut child_rects = Vec::with_capacity(children.len());
        let mut y = 0.0;
        for r in 0..row_count {
            let mut x = 0.0;
            for c in 0..col_count {
                let child_idx = r * col_count + c;
                if child_idx < children.len() {
                    let child_width = col_widths[c];
                    let child_height = row_heights[r];

                    let child_constraints = Constraints::tight(child_width, child_height);
                    let _ = children[child_idx].layout(child_constraints, &[], text_system, theme);

                    child_rects.push(Rect {
                        x,
                        y,
                        width: child_width,
                        height: child_height,
                    });
                }
                x += col_widths[c] + self.column_gap;
            }
            y += row_heights[r] + self.row_gap;
        }

        (LayoutResult { size: final_size }, child_rects)
    }
}
