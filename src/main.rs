mod double_pendulum;

use double_pendulum::State;
use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Double Pendulum Simulation".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

fn draw_cartesian_plane() {
    let grid_color = Color::from_rgba(45, 49, 62, 255);
    let axis_color = Color::from_rgba(105, 115, 140, 255);

    let spacing = 40.0;
    let cx = SCREEN_WIDTH / 2.0;
    let cy = SCREEN_HEIGHT / 2.0;

    let mut x = cx;
    while x <= SCREEN_WIDTH {
        draw_line(x, 0.0, x, SCREEN_HEIGHT, 1.0, grid_color);
        x += spacing;
    }

    x = cx - spacing;
    while x >= 0.0 {
        draw_line(x, 0.0, x, SCREEN_HEIGHT, 1.0, grid_color);
        x -= spacing;
    }

    let mut y = cy;
    while y <= SCREEN_HEIGHT {
        draw_line(0.0, y, SCREEN_WIDTH, y, 1.0, grid_color);
        y += spacing;
    }

    y = cy - spacing;
    while y >= 0.0 {
        draw_line(0.0, y, SCREEN_WIDTH, y, 1.0, grid_color);
        y -= spacing;
    }

    draw_line(cx, 0.0, cx, SCREEN_HEIGHT, 2.0, axis_color);
    draw_line(0.0, cy, SCREEN_WIDTH, cy, 2.0, axis_color);
}

#[macroquad::main(window_conf)]
async fn main() {
    let scale = miniquad::window::dpi_scale();

    if (scale - 1.0).abs() > f32::EPSILON {
        let scaled_w = (SCREEN_WIDTH * scale).round() as u32;
        let scaled_h = (SCREEN_HEIGHT * scale).round() as u32;
        miniquad::window::set_window_size(scaled_w, scaled_h);
    }

    let l1: f64 = 1.5;
    let l2: f64 = 1.5;

    let m1: f64 = 1.0;
    let m2: f64 = 1.0;

    let mut state = State {
        theta1: 1.0,
        omega1: 4.0,
        theta2: 0.5,
        omega2: -2.0,
    };

    let bg_color = Color::from_rgba(18, 20, 28, 255);
    let rod_color = Color::from_rgba(75, 155, 255, 255);
    let bob_color = Color::from_rgba(255, 75, 85, 255);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Sub-step fixed physics per frame to avoid RK4 explosion during frame drops
        let dt = (get_frame_time() as f64).min(0.05);
        let substeps = 10;
        let sub_dt = dt / substeps as f64;

        for _ in 0..substeps {
            double_pendulum::rk4_step(l1, m1, l2, m2, &mut state, sub_dt);
        }

        let x0 = (SCREEN_WIDTH / 2.0) as f64;
        let y0 = 100.0;

        let x1 = x0 + l1 * state.theta1.sin() * 100.0;
        let y1 = y0 + l1 * state.theta1.cos() * 100.0;

        let x2 = x1 + l2 * state.theta2.sin() * 100.0;
        let y2 = y1 + l2 * state.theta2.cos() * 100.0;

        clear_background(bg_color);
        draw_cartesian_plane();

        let radius = 8.0;

        draw_line(
            x0 as f32, y0 as f32,
            x1 as f32, y1 as f32,
            2.5, rod_color,
        );

        draw_line(
            x1 as f32, y1 as f32,
            x2 as f32, y2 as f32,
            2.5, rod_color,
        );

        draw_circle(x0 as f32, y0 as f32, radius, bob_color);
        draw_circle(x1 as f32, y1 as f32, radius, bob_color);
        draw_circle(x2 as f32, y2 as f32, radius, bob_color);

        next_frame().await;
    }
}
