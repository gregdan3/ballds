extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Bouncing Circle", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut x_pos = 50.0;
    let mut y_pos = 50.0;
    let mut x_vel = 15.0;
    let mut y_vel = 15.0;
    let radius = 25.0;

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0, 0.0, 1.0, 1.0], graphics);

                ellipse(
                    [1.0, 1.0, 1.0, 1.0], // white
                    [x_pos - radius, y_pos - radius, radius * 2.0, radius * 2.0],
                    context.transform,
                    graphics,
                );
            });
        }

        if let Some(update) = event.update_args() {
            x_pos += x_vel * update.dt;
            y_pos += y_vel * update.dt;

            if x_pos + radius > window.size().width || x_pos - radius < 0.0 {
                x_vel = -x_vel;
            }

            if y_pos + radius > window.size().height || y_pos - radius < 0.0 {
                y_vel = -y_vel;
            }
        }
    }
}
