use super::{FlowField, Grid};

pub struct Agent {
    pub x: usize,
    pub y: usize,
}

impl Agent {
    pub fn new(x: usize, y: usize) -> Self {
        Agent { x, y }
    }

    pub fn target_pos(&mut self, grid: &Grid, field: &FlowField) -> (i32, i32) {
        return field.get_direction_at(self.x, self.y);
    }
}
