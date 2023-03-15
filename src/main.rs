extern crate piston_window;
extern crate rand;

use piston_window::*;

mod balls;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Balls", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut balls: Vec<Box<balls::Ball>> = vec![];
    let mut last_mouse_pos: [f64; 2] = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            balls::render_balls(&balls, &mut window, &event);
        }

        if let Some(pos) = event.mouse_cursor_args() {
            last_mouse_pos = pos;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            for _ in 0..10 {
                let ball = Box::new(balls::gen_ball(last_mouse_pos));
                balls.push(ball);
            }
        }

        if let Some(update) = event.update_args() {
            balls::update_balls(
                &mut balls,
                update.dt,
                window.size().width,
                window.size().height,
            );
        }
    }
}
