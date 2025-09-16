pub mod agent;
pub mod field;
pub mod grid;
pub use agent::Agent;
pub use field::FlowField;
pub use grid::Grid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_movement() {
        let grid = grid::Grid::new(10, 10);
        let field = field::FlowField::new(&grid, (5, 5));
        let mut agent = agent::Agent::new(0, 0);
        let result = field.get_direction_at(agent.x, agent.y);
        assert_eq!(result, (1,1));
    }

    #[test]
    fn cost_function() {
        let grid = grid::Grid::new(10, 10);
        let field = field::FlowField::new(&grid, (5, 5));
        let result = field.get_cost_at(0, 0);
        assert_eq!(result, 70);
    }
}
