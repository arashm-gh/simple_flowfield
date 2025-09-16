use crate::grid::Grid;
use std::collections::VecDeque;

pub struct FlowField {
    pub width: usize,
    pub height: usize,
    pub costs: Vec<Vec<i32>>,
    pub directions: Vec<Vec<(i32, i32)>>,
}

impl FlowField {
    pub fn new(grid: &Grid, target: (usize, usize)) -> Self {
        let (w, h) = (grid.width, grid.height);

        // 1. Initialize costs
        let mut costs = vec![vec![i32::MAX; w as usize]; h as usize];
        costs[target.1][target.0] = 0;

        // 2. BFS queue
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

        // 3. Compute flow directions
        let mut directions = vec![vec![(0, 0); w as usize]; h as usize];
        for y in 0..h as usize {
            for x in 0..w as usize {
                if grid.is_obstacle(x as i32, y as i32) || costs[y][x] == i32::MAX {
                    continue; // leave as (0,0)
                }

                // Pick neighbor with lowest cost
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
