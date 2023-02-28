extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::prelude::*;

struct Ball {
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    radius: f64,
    color: [f32; 4],
}

fn gen_ball(pos: [f64; 2]) -> Ball {
    let mut rng = rand::thread_rng();
    let radius = rng.gen_range(10.0..50.0);
    let x_vel = rng.gen_range(-50.0..50.0);
    let y_vel = rng.gen_range(-50.0..50.0);
    let color = [
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
        rng.gen_range(0.0..1.0),
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

fn update_balls(balls: &mut Vec<Ball>, dt: f64, window_width: f64, window_height: f64) {
    for ball in balls {
        ball.x_pos += ball.x_vel * dt;
        ball.y_pos += ball.y_vel * dt;

        if ball.x_pos + ball.radius > window_width || ball.x_pos - ball.radius < 0.0 {
            ball.x_vel = -ball.x_vel;
        }

        if ball.y_pos + ball.radius > window_height || ball.y_pos - ball.radius < 0.0 {
            ball.y_vel = -ball.y_vel;
        }
    }
}

fn render_balls(balls: &Vec<Ball>, window: &mut PistonWindow, event: &Event) {
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

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Balls", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut balls: Vec<Ball> = vec![];
    let mut last_mouse_pos: [f64; 2] = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            render_balls(&balls, &mut window, &event);
        }

        if let Some(pos) = event.mouse_cursor_args() {
            last_mouse_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            let ball = gen_ball(last_mouse_pos);
            balls.push(ball);
        }

        if let Some(update) = event.update_args() {
            update_balls(
                &mut balls,
                update.dt,
                window.size().width,
                window.size().height,
            );
        }
    }
}
