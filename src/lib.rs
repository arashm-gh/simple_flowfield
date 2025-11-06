pub mod agent;
pub mod field;
pub mod grid;
pub use agent::{Agent2D, Agent3D};
pub use field::{FlowField2D, FlowField3D};
pub use grid::{Grid2D, Grid3D};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_movement() {
        let grid = grid::Grid2D::new(10, 10);
        let field = field::FlowField2D::new(&grid, (5, 5));
        let mut agent = agent::Agent2D::new(0, 0);
        let result = field.get_direction_at(agent.x, agent.y);
        assert_eq!(result, (1, 1));
    }

    #[test]
    fn cost_function() {
        let grid = grid::Grid2D::new(10, 10);
        let field = field::FlowField2D::new(&grid, (5, 5));
        let result = field.get_cost_at(0, 0);
        assert_eq!(result, 70);
    }
}
