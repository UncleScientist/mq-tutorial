use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 200.0;

#[macroquad::main("My game")]
async fn main() {
    let mut x = screen_width() / 2.;
    let mut y = screen_height() / 2.;

    loop {
        let delta_time = get_frame_time();
        let circle_movement = MOVEMENT_SPEED * delta_time;

        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            x += circle_movement;
        }
        if is_key_down(KeyCode::Left) {
            x -= circle_movement;
        }
        if is_key_down(KeyCode::Down) {
            y += circle_movement;
        }
        if is_key_down(KeyCode::Up) {
            y -= circle_movement;
        }

        x = clamp(x, 0.0, screen_width());
        y = clamp(y, 0.0, screen_height());

        draw_circle(x, y, 16.0, YELLOW);

        next_frame().await
    }
}
