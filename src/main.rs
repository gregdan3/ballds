extern crate piston_window;

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

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Balls", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut balls = vec![
        Ball {
            x_pos: 50.0,
            y_pos: 50.0,
            x_vel: 15.0,
            y_vel: 15.0,
            radius: 25.0,
            color: [1.0, 1.0, 1.0, 1.0], // white
        },
        Ball {
            x_pos: 200.0,
            y_pos: 100.0,
            x_vel: -10.0,
            y_vel: 5.0,
            radius: 40.0,
            color: [1.0, 0.0, 0.0, 1.0], // red
        },
        Ball {
            x_pos: 300.0,
            y_pos: 250.0,
            x_vel: 8.0,
            y_vel: -12.0,
            radius: 20.0,
            color: [0.0, 1.0, 0.0, 1.0], // green
        },
    ];

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0, 0.0, 1.0, 1.0], graphics);

                for ball in &balls {
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

        if let Some(update) = event.update_args() {
            for ball in &mut balls {
                ball.x_pos += ball.x_vel * update.dt;
                ball.y_pos += ball.y_vel * update.dt;

                if ball.x_pos + ball.radius > window.size().width || ball.x_pos - ball.radius < 0.0
                {
                    ball.x_vel = -ball.x_vel;
                }

                if ball.y_pos + ball.radius > window.size().height || ball.y_pos - ball.radius < 0.0
                {
                    ball.y_vel = -ball.y_vel;
                }
            }
        }
    }
}
