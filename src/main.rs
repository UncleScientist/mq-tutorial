use macroquad::prelude::*;

const MOVEMENT_SPEED: f32 = 200.0;

#[macroquad::main("My game")]
async fn main() {
    let mut squares = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.,
        y: screen_height() / 2.,
    };

    rand::srand(miniquad::date::now() as u64);

    loop {
        let delta_time = get_frame_time();
        let circle_movement = MOVEMENT_SPEED * delta_time;

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                y: -size,
            });
        }

        clear_background(DARKPURPLE);

        if is_key_down(KeyCode::Right) {
            circle.x += circle_movement;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle_movement;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle_movement;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle_movement;
        }

        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());

        for square in &mut squares {
            square.y += square.speed * delta_time;
        }

        squares.retain(|square| square.y < screen_height() + square.size);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                GREEN,
            );
        }

        draw_circle(circle.x, circle.y, 16.0, YELLOW);

        next_frame().await
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}
