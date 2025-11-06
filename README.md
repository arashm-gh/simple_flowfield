# flowfield

flowfield is a rust crate for quickly and easily implementing a FlowField pathfinding system into your project.  
it works for both 3d and 2d!

## Installation

Use:

```bash
cargo add simple_flowfield
```

## Usage

```rust
use simple_flowfield::*; 

// define the entire grid map 
let grid = grid::Grid2D::new(100, 100);
// define the flowfield for a single target
let fieldA = field::FlowField2D::new(&grid, (49, 91));
// initalize an agent
let mut agent = agent::Agent2D::new(0, 0);

// returns a (i32, i32)
let result = fieldA.get_direction_at(agent.x, agent.y);

// add obstacles in the grid
grid.set_rect_obstacle(40, 0, 3, 34, true);

// every agent can work with a different flow field
let fieldB = field::FlowField2D::new(&grid, (0, 0));
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.
