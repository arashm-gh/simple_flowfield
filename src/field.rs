use crate::grid::{Grid2D, Grid3D};
use std::collections::VecDeque;

pub struct FlowField2D {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<i32>>,
    pub directions: Vec<Vec<(i32, i32)>>,
}

pub struct FlowField3D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub costs: Vec<Vec<Vec<i32>>>,
    pub directions: Vec<Vec<Vec<(i32, i32, i32)>>>,
}

impl FlowField2D {
    pub fn new(grid: &Grid2D, target: (usize, usize)) -> Self {
        let (w, h) = (grid.width, grid.height);

        // init costs
        let mut costs = vec![vec![i32::MAX; w as usize]; h as usize];
        costs[target.1][target.0] = 0;

        // breadth-first search queue
        let mut queue = VecDeque::new();
        queue.push_back(target);

        let dirs = [
            (1, 0, 10),
            (-1, 0, 10),
            (0, 1, 10),
            (0, -1, 10), // straight = 10
            (1, 1, 14),
            (1, -1, 14),
            (-1, 1, 14),
            (-1, -1, 14), // diagonal = 14
        ];

        while let Some((x, y)) = queue.pop_front() {
            let current_cost = costs[y][x];
            for (dx, dy, step_cost) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && ny >= 0 && nx < w as i32 && ny < h as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);

                    if grid.is_obstacle(nx as i32, ny as i32) {
                        continue;
                    }

                    let new_cost = current_cost + step_cost;

                    if costs[ny][nx] > new_cost {
                        costs[ny][nx] = new_cost;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        // flow directions
        let mut directions = vec![vec![(0, 0); w as usize]; h as usize];
        for y in 0..h as usize {
            for x in 0..w as usize {
                if grid.is_obstacle(x as i32, y as i32) || costs[y][x] == i32::MAX {
                    continue; // leave as (0,0)
                }

                // neighbor with lowest cost
                let mut best_dir = (0, 0);
                let mut best_cost = costs[y][x];

                for (dx, dy, _step_cost) in dirs.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;

                    if nx >= 0 && ny >= 0 && nx < w as i32 && ny < h as i32 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if costs[ny][nx] < best_cost {
                            best_cost = costs[ny][nx];
                            best_dir = (*dx, *dy);
                        }
                    }
                }

                directions[y][x] = best_dir;
            }
        }

        Self {
            width: w as usize,
            height: h as usize,
            costs,
            directions,
        }
    }

    pub fn get_direction_at(&self, x: usize, y: usize) -> (i32, i32) {
        self.directions[y][x]
    }

    pub fn get_cost_at(&self, x: usize, y: usize) -> i32 {
        self.costs[y][x]
    }
}

impl FlowField3D {
    pub fn new(grid: &Grid3D, target: (usize, usize, usize)) -> Self {
        let (w, h, d) = (grid.width, grid.height, grid.depth);

        // initialize costs
        let mut costs = vec![vec![vec![i32::MAX; w]; h]; d];
        costs[target.2][target.1][target.0] = 0;

        // BFS queue
        let mut queue = VecDeque::new();
        queue.push_back(target);

        // 26-direction neighborhood
        let dirs = [
            // 6 faces
            (1, 0, 0, 10),
            (-1, 0, 0, 10),
            (0, 1, 0, 10),
            (0, -1, 0, 10),
            (0, 0, 1, 10),
            (0, 0, -1, 10),
            // 12 edges
            (1, 1, 0, 14),
            (1, -1, 0, 14),
            (-1, 1, 0, 14),
            (-1, -1, 0, 14),
            (1, 0, 1, 14),
            (1, 0, -1, 14),
            (-1, 0, 1, 14),
            (-1, 0, -1, 14),
            (0, 1, 1, 14),
            (0, 1, -1, 14),
            (0, -1, 1, 14),
            (0, -1, -1, 14),
            // 8 corners
            (1, 1, 1, 17),
            (1, 1, -1, 17),
            (1, -1, 1, 17),
            (-1, 1, 1, 17),
            (-1, -1, 1, 17),
            (1, -1, -1, 17),
            (-1, 1, -1, 17),
            (-1, -1, -1, 17),
        ];

        // BFS cost propagation
        while let Some((x, y, z)) = queue.pop_front() {
            let current_cost = costs[z][y][x];
            for (dx, dy, dz, step_cost) in dirs.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                let nz = z as i32 + dz;

                if nx >= 0 && ny >= 0 && nz >= 0 && nx < w as i32 && ny < h as i32 && nz < d as i32
                {
                    let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);

                    if grid.is_obstacle(nx as i32, ny as i32, nz as i32) {
                        continue;
                    }

                    let new_cost = current_cost + step_cost;

                    if costs[nz][ny][nx] > new_cost {
                        costs[nz][ny][nx] = new_cost;
                        queue.push_back((nx, ny, nz));
                    }
                }
            }
        }

        // compute flow directions
        let mut directions = vec![vec![vec![(0, 0, 0); w]; h]; d];
        for z in 0..d {
            for y in 0..h {
                for x in 0..w {
                    if grid.is_obstacle(x as i32, y as i32, z as i32) || costs[z][y][x] == i32::MAX
                    {
                        continue;
                    }

                    let mut best_dir = (0, 0, 0);
                    let mut best_cost = costs[z][y][x];

                    for (dx, dy, dz, _) in dirs.iter() {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let nz = z as i32 + dz;

                        if nx >= 0
                            && ny >= 0
                            && nz >= 0
                            && nx < w as i32
                            && ny < h as i32
                            && nz < d as i32
                        {
                            let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
                            if costs[nz][ny][nx] < best_cost {
                                best_cost = costs[nz][ny][nx];
                                best_dir = (*dx, *dy, *dz);
                            }
                        }
                    }

                    directions[z][y][x] = best_dir;
                }
            }
        }

        Self {
            width: w,
            height: h,
            depth: d,
            costs,
            directions,
        }
    }

    pub fn get_direction_at(&self, x: usize, y: usize, z: usize) -> (i32, i32, i32) {
        self.directions[z][y][x]
    }

    pub fn get_cost_at(&self, x: usize, y: usize, z: usize) -> i32 {
        self.costs[z][y][x]
    }
}
