use raylib::prelude::*;
use simple_flowfield::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1600, 900)
        .title("FlowField3D Visualization")
        .build();

    let mut cam = Camera3D::perspective(
        Vector3::new(30.0, 30.0, 30.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    let mut grid = Grid3D::new(50, 30, 50);
    grid.set_cuboid_obstacle(10, 0, 10, 30, 8, 30, true);

    let field = FlowField3D::new(&grid, (40, 10, 40));
    let mut agent = Agent3D::new(5, 20, 5);

    let mut prev_pos = (agent.x, agent.y, agent.z);

    let voxel_scale = 1.0;
    let half_w = grid.width as f32 / 2.0;
    let half_d = grid.depth as f32 / 2.0;

    rl.set_target_fps(6);
    while !rl.window_should_close() {
        // Step movement
        prev_pos = (agent.x, agent.y, agent.z);

        let (nx, ny, nz) = field.get_direction_at(agent.x, agent.y, agent.z);
        let (x, y, z) = (
            (agent.x as i32 + nx).clamp(0, grid.width as i32 - 1),
            (agent.y as i32 + ny).clamp(0, grid.height as i32 - 1),
            (agent.z as i32 + nz).clamp(0, grid.depth as i32 - 1),
        );
        if !grid.is_obstacle(x, y, z) {
            agent.x = x as usize;
            agent.y = y as usize;
            agent.z = z as usize;
        }

        // Compute positions
        let prev_agent_pos = Vector3::new(
            (prev_pos.0 as f32 - half_w) * voxel_scale,
            prev_pos.1 as f32 * voxel_scale,
            (prev_pos.2 as f32 - half_d) * voxel_scale,
        );
        let agent_pos = Vector3::new(
            (agent.x as f32 - half_w) * voxel_scale,
            agent.y as f32 * voxel_scale,
            (agent.z as f32 - half_d) * voxel_scale,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::DARKGRAY);
        let mut d3d = d.begin_mode3D(cam);

        // Draw world
        for z in 0..grid.depth {
            for y in 0..grid.height {
                for x in 0..grid.width {
                    if !grid.is_obstacle(x as i32, y as i32, z as i32) {
                        continue;
                    }
                    let wx = (x as f32 - half_w) * voxel_scale;
                    let wy = y as f32 * voxel_scale;
                    let wz = (z as f32 - half_d) * voxel_scale;
                    let pos = Vector3::new(wx, wy, wz);

                    d3d.draw_cube(
                        pos,
                        voxel_scale - 0.05,
                        voxel_scale - 0.05,
                        voxel_scale - 0.05,
                        Color::RED,
                    );
                    d3d.draw_cube_wires(pos, voxel_scale, voxel_scale, voxel_scale, Color::BLACK);
                }
            }
        }

        // Draw current and previous radii
        let draw_radius = |d3d: &mut RaylibMode3D<RaylibDrawHandle>,
                           center: (usize, usize, usize),
                           color: Color| {
            let radius = 3;
            let base_y = (center.1 as f32 - 1.0).max(0.0);
            for dz in -radius..=radius {
                for dx in -radius..=radius {
                    if (dx * dx + dz * dz) as f32 > (radius as f32).powi(2) {
                        continue;
                    }
                    let bx = (center.0 as i32 + dx).clamp(0, grid.width as i32 - 1);
                    let bz = (center.2 as i32 + dz).clamp(0, grid.depth as i32 - 1);
                    let by = base_y as i32;
                    if grid.is_obstacle(bx, by, bz) {
                        continue;
                    }
                    let wx = (bx as f32 - half_w) * voxel_scale;
                    let wy = by as f32 * voxel_scale;
                    let wz = (bz as f32 - half_d) * voxel_scale;
                    let pos = Vector3::new(wx, wy, wz);
                    d3d.draw_cube(
                        pos,
                        voxel_scale - 0.05,
                        voxel_scale - 0.05,
                        voxel_scale - 0.05,
                        color,
                    );
                    d3d.draw_cube_wires(pos, voxel_scale, voxel_scale, voxel_scale, Color::BLACK);
                }
            }
        };

        // Previous step radius in a dimmer green
        draw_radius(&mut d3d, prev_pos, Color::DARKGREEN);
        // Current step radius in bright lime
        draw_radius(&mut d3d, (agent.x, agent.y, agent.z), Color::LIME);

        // Agent cube
        d3d.draw_cube(
            agent_pos,
            voxel_scale,
            voxel_scale,
            voxel_scale,
            Color::BLUE,
        );

        // Direction arrow
        let dir = Vector3::new(nx as f32, ny as f32, nz as f32);
        if dir.length() > 0.0 {
            let arrow_start = agent_pos + Vector3::new(0.0, voxel_scale * 0.5, 0.0);
            let arrow_end = arrow_start + dir * (voxel_scale * 2.0);
            d3d.draw_cylinder_ex(arrow_start, arrow_end, 0.05, 0.05, 8, Color::YELLOW);
            let head_start = arrow_end - dir * 0.2;
            d3d.draw_cylinder_ex(head_start, arrow_end, 0.15, 0.0, 8, Color::ORANGE);
        }

        drop(d3d);
        d.draw_text("FlowField3D", 10, 10, 20, Color::WHITE);
    }
}
