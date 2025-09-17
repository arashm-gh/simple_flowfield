use simple_flowfield::*;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1920, 1080)
        .title("FlowField Usage")
        .build();

    let mut grid = simple_flowfield::Grid::new(100, 100);
    grid.set_rect_obstacle(40, 0, 3, 60, true);
    grid.set_rect_obstacle(40, 61, 3, 34, true);

    let field = simple_flowfield::FlowField::new(&grid, (31, 10));
    let mut agent = Agent::new(67, 30);
    rl.set_target_fps(12);
    while !rl.window_should_close() {
        // input
        let (nx, ny) = field.get_direction_at(agent.x, agent.y);
        agent.x = (agent.x as i32 + nx) as usize;
        agent.y = (agent.y as i32 + ny) as usize;
        // update

        // draw
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);

        // draw grid
        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.is_obstacle(x as i32, y as i32) {
                    d.draw_rectangle(x as i32 * 16, y as i32 * 16, 16, 16, Color::RED);
                } else {
                    d.draw_rectangle(x as i32 * 16, y as i32 * 16, 16, 16, Color::GRAY);
                    d.draw_text(
                        format!("{}", field.get_cost_at(x as usize, y as usize)).as_str(),
                        x as i32 * 16,
                        y as i32 * 16,
                        14,
                        Color::BLACK,
                    );
                }
            }
        }

        d.draw_rectangle(
            agent.x as i32 * 16,
            agent.y as i32 * 16,
            15,
            15,
            Color::BLUE,
        );
    }
}
