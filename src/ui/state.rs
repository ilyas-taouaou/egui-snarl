use egui::{style::Spacing, vec2, Context, Frame, Id, Pos2, Rect, Style, Vec2};

use super::Zoom;

/// Node UI state.
#[derive(Clone, Copy, PartialEq)]
pub struct NodeState {
    /// Size occupied by title.
    pub title_size: Vec2,

    /// Size occupied by inputs.
    pub inputs_size: Vec2,

    /// Size occupied by outputs.
    pub outputs_size: Vec2,
}

impl NodeState {
    pub fn load(cx: &Context, id: Id) -> Option<Self> {
        cx.data_mut(|d| d.get_temp(id))
    }

    pub fn store(&self, cx: &Context, id: Id) {
        cx.data_mut(|d| d.insert_temp(id, *self));
    }

    /// Finds node rect at specific position (excluding node frame margin).
    pub fn node_rect(&self, frame: &Frame, spacing: &Spacing, pos: Pos2) -> Rect {
        let width = self
            .title_size
            .x
            .max(self.inputs_size.x + spacing.item_spacing.x + self.outputs_size.x);

        let height = self.title_size.y
            + frame.total_margin().bottom
            + frame.total_margin().bottom
            + self.inputs_size.y.max(self.outputs_size.y);

        Rect::from_min_size(pos, vec2(width, height))
    }

    /// Finds title rect at specific position (excluding node frame margin).
    pub fn title_rect(&self, spacing: &Spacing, pos: Pos2) -> Rect {
        let width = self
            .title_size
            .x
            .max(self.inputs_size.x + spacing.item_spacing.x + self.outputs_size.x);

        let height = self.title_size.y;

        Rect::from_min_size(pos, vec2(width, height))
    }

    /// Finds pins rect at specific position (excluding node frame margin).
    pub fn pins_rect(&self, frame: &Frame, spacing: &Spacing, openness: f32, pos: Pos2) -> Rect {
        let height = self.inputs_size.y.max(self.outputs_size.y);
        let width = self
            .title_size
            .x
            .max(self.inputs_size.x + spacing.item_spacing.x + self.outputs_size.x);

        let moved =
            (height + frame.total_margin().bottom + frame.total_margin().bottom) * (openness - 1.0);

        let pos = pos
            + vec2(
                0.0,
                self.title_size.y
                    + frame.total_margin().bottom
                    + frame.total_margin().bottom
                    + moved,
            );

        Rect::from_min_size(pos, vec2(width, height))
    }

    pub fn initial(spacing: &Spacing) -> Self {
        NodeState {
            title_size: spacing.interact_size,
            inputs_size: spacing.interact_size,
            outputs_size: spacing.interact_size,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnarlState {
    /// Where viewport's left-top in graph's space.
    offset: Vec2,

    /// Scale of the viewport.
    scale: f32,
}

impl Default for SnarlState {
    fn default() -> Self {
        SnarlState {
            offset: Vec2::ZERO,
            scale: 1.0,
        }
    }
}

pub struct ZoomState {
    pub offset: Vec2,
    pub scale: f32,
}

impl ZoomState {
    pub fn graph_point_to_screen(&self, point: Pos2, viewport: Rect) -> Pos2 {
        point * self.scale - self.offset + viewport.min.to_vec2()
    }

    pub fn zoom_style(&self, style: &mut Style) {
        style.zoom(self.scale);
    }
}

impl SnarlState {
    pub fn load(cx: &Context, id: Id) -> Option<Self> {
        cx.data_mut(|d| d.get_temp(id))
    }

    pub fn store(&self, cx: &Context, id: Id) {
        cx.data_mut(|d| d.insert_temp(id, *self));
    }

    pub fn get_zoom(&self, id: Id, cx: &Context, style: &Style) -> ZoomState {
        let x = cx.animate_value_with_time(
            id.with("zoom-offset-x"),
            self.offset.x,
            style.animation_time,
        );
        let y = cx.animate_value_with_time(
            id.with("zoom-offset-y"),
            self.offset.y,
            style.animation_time,
        );

        let scale =
            cx.animate_value_with_time(id.with("zoom-scale"), self.scale, style.animation_time);

        ZoomState {
            offset: vec2(x, y),
            scale,
        }
    }

    pub fn apply_scale_wrt_screen_point(&mut self, delta_scale: f32, pivot: Pos2, viewport: Rect) {
        let a = pivot + self.offset - viewport.min.to_vec2();

        self.offset += a * delta_scale - a;
        self.scale *= delta_scale;
    }

    // fn screen_point_to_graph(&self, point: Pos2, viewport: Rect) -> Pos2 {
    //     (point + self.offset - viewport.min.to_vec2()) / self.scale
    // }

    // fn graph_size_to_screen(&self, size: Vec2) -> Vec2 {
    //     size * self.scale
    // }

    // fn screen_size_to_graph(&self, size: Vec2) -> Vec2 {
    //     size / self.scale
    // }

    // fn graph_distance_to_screen(&self, distance: f32) -> f32 {
    //     distance * self.scale
    // }

    // fn screen_distance_to_graph(&self, distance: f32) -> f32 {
    //     distance / self.scale
    // }
}