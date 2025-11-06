use super::{FlowField2D, FlowField3D, Grid2D, Grid3D};

pub struct Agent2D {
    pub x: usize,
    pub y: usize,
}

impl Agent2D {
    pub fn new(x: usize, y: usize) -> Self {
        Agent2D { x, y }
    }

    pub fn target_pos(&mut self, grid: &Grid2D, field: &FlowField2D) -> (i32, i32) {
        return field.get_direction_at(self.x, self.y);
    }
}

pub struct Agent3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Agent3D {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Agent3D { x, y, z }
    }

    pub fn target_pos(&mut self, grid: &Grid3D, field: &FlowField3D) -> (i32, i32, i32) {
        return field.get_direction_at(self.x, self.y, self.z);
    }
}
