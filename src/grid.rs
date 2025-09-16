pub struct Tile {
    pub is_obstacle: bool,
}

impl Tile {
    pub fn new() -> Self {
        Tile { is_obstacle: false }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = (0..(width * height)).map(|_| Tile::new()).collect();

        Grid {
            width,
            height,
            tiles,
        }
    }

    /// Convert (x,y) to flat index
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    /// Mark a cell as an obstacle
    pub fn set_obstacle(&mut self, x: usize, y: usize, value: bool) {
        let idx = self.idx(x, y);
        self.tiles[idx].is_obstacle = value;
    }

    pub fn set_rect_obstacle(&mut self, x: usize, y: usize, w: usize, h: usize, value: bool) {
        let max_x = (x + w).min(self.width);
        let max_y = (y + h).min(self.height);

        for yy in y..max_y {
            for xx in x..max_x {
                self.set_obstacle(xx, yy, value);
            }
        }
    }

    /// Query if a cell is an obstacle
    pub fn is_obstacle(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return true; // treat out of bounds as obstacles
        }
        let idx = self.idx(x as usize, y as usize);
        self.tiles[idx].is_obstacle
    }
}
