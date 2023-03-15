extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::prelude::*;

pub struct Ball {
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    radius: f64,
    color: [f32; 4],
}

pub fn gen_ball(pos: [f64; 2]) -> Ball {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(10.0..30.0);
    let x_vel = rng.gen_range(-50.0..50.0);
    let y_vel = rng.gen_range(-50.0..50.0);
    let color = [
        rng.gen_range(0.1..1.0),
        rng.gen_range(0.1..1.0),
        rng.gen_range(0.1..1.0),
        1.0,
    ];
    Ball {
        x_pos: pos[0],
        y_pos: pos[1],
        x_vel,
        y_vel,
        radius,
        color,
    }
}

fn move_ball_by_vel(ball: &mut Ball, dt: f64) {
    ball.x_pos += ball.x_vel * dt;
    ball.y_pos += ball.y_vel * dt;
}

fn try_bump_balls(ball: &mut Ball, other: &mut Ball) {
    let distance = ((ball.x_pos - other.x_pos).powi(2) + (ball.y_pos - other.y_pos).powi(2)).sqrt();

    if distance <= ball.radius + other.radius {
        // mass proportional to volume (radius^2)
        let m1 = std::f64::consts::PI * ball.radius.powi(2);
        let m2 = std::f64::consts::PI * other.radius.powi(2);

        let vx_ball = ball.x_vel;
        let vy_ball = ball.y_vel;
        let vx_other = other.x_vel;
        let vy_other = other.y_vel;

        ball.x_vel = ((m1 - m2) * vx_ball + 2.0 * m2 * vx_other) / (m1 + m2);
        ball.y_vel = ((m1 - m2) * vy_ball + 2.0 * m2 * vy_other) / (m1 + m2);
        other.x_vel = ((m2 - m1) * vx_other + 2.0 * m1 * vx_ball) / (m1 + m2);
        other.y_vel = ((m2 - m1) * vy_other + 2.0 * m1 * vy_ball) / (m1 + m2);

        let overlap = ball.radius + other.radius - distance;
        let dx = (ball.x_pos - other.x_pos) / distance;
        let dy = (ball.y_pos - other.y_pos) / distance;

        let repulsion_strength = 0.5;
        let repulsion_force = repulsion_strength * overlap;

        ball.x_pos += repulsion_force * dx;
        ball.y_pos += repulsion_force * dy;
        other.x_pos -= repulsion_force * dx;
        other.y_pos -= repulsion_force * dy;
    }
}

fn try_bump_walls(ball: &mut Ball, window_width: f64, window_height: f64) {
    if ball.x_pos + ball.radius > window_width {
        ball.x_vel = -ball.x_vel;
        ball.x_pos = window_width - ball.radius;
    } else if ball.x_pos - ball.radius < 0.0 {
        ball.x_vel = -ball.x_vel;
        ball.x_pos = ball.radius;
    }

    if ball.y_pos + ball.radius > window_height {
        ball.y_vel = -ball.y_vel;
        ball.y_pos = window_height - ball.radius;
    } else if ball.y_pos - ball.radius < 0.0 {
        ball.y_vel = -ball.y_vel;
        ball.y_pos = ball.radius;
    }
}

pub fn update_balls(balls: &mut [Box<Ball>], dt: f64, window_width: f64, window_height: f64) {
    let l = balls.len();

    for i in 0..l {
        let (left, right) = balls.split_at_mut(i + 1);
        let ball = &mut *left[i];
        move_ball_by_vel(ball, dt);
        for other in right {
            try_bump_balls(ball, other);
        }
        try_bump_walls(ball, window_width, window_height)
    }
}

pub fn render_balls(balls: &[Box<Ball>], window: &mut PistonWindow, event: &Event) {
    window.draw_2d(event, |context, graphics, _device| {
        clear([0.0, 0.0, 0.0, 1.0], graphics);

        for ball in balls {
            ellipse(
                ball.color,
                [
                    ball.x_pos - ball.radius,
                    ball.y_pos - ball.radius,
                    ball.radius * 2.0,
                    ball.radius * 2.0,
                ],
                context.transform,
                graphics,
            );
        }
    });
}
