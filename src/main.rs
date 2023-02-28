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

fn gen_ball(balls: &mut Vec<Ball>, pos: [f64; 2]) {
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
    balls.push(Ball {
        x_pos: pos[0],
        y_pos: pos[1],
        x_vel,
        y_vel,
        radius,
        color,
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

        if let Some(pos) = event.mouse_cursor_args() {
            last_mouse_pos = pos;
        } else {
            println!("No mouse position data");
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            gen_ball(&mut balls, last_mouse_pos)
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
