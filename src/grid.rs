pub struct Tile {
    pub is_obstacle: bool,
}

impl Tile {
    pub fn new() -> Self {
        Tile { is_obstacle: false }
    }
}

pub struct Grid2D {
    pub width: usize,
    pub height: usize,
    tiles: Vec<Tile>,
}

pub struct Grid3D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    tiles: Vec<Tile>,
}

impl Grid2D {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = (0..(width * height)).map(|_| Tile::new()).collect();

        Grid2D {
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

impl Grid3D {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        let tiles = (0..(width * height * depth)).map(|_| Tile::new()).collect();

        Grid3D {
            width,
            height,
            depth,
            tiles,
        }
    }

    pub fn idx(&self, x: usize, y: usize, z: usize) -> usize {
        z * (self.width * self.height) + y * self.width + x
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize, z: usize, value: bool) {
        let idx = self.idx(x, y, z);
        self.tiles[idx].is_obstacle = value;
    }

    pub fn set_cuboid_obstacle(
        &mut self,
        x: usize,
        y: usize,
        z: usize,
        w: usize,
        h: usize,
        d: usize,
        value: bool,
    ) {
        let max_x = (x + w).min(self.width);
        let max_y = (y + h).min(self.height);
        let max_z = (z + d).min(self.depth);

        for zz in z..max_z {
            for yy in y..max_y {
                for xx in x..max_x {
                    self.set_obstacle(xx, yy, zz, value);
                }
            }
        }
    }

    pub fn is_obstacle(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0
            || y < 0
            || z < 0
            || x as usize >= self.width
            || y as usize >= self.height
            || z as usize >= self.depth
        {
            return true; // treat out of bounds as obstacles
        }
        let idx = self.idx(x as usize, y as usize, z as usize);
        self.tiles[idx].is_obstacle
    }
}
