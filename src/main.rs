use macroquad::prelude::*;
use rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 200.0;

const MAX_BULLETS: usize = 7;
const BULLET_COOLDOWN: f64 = 0.25;

const COLOR_LIST: [Color; 20] = [
    LIGHTGRAY, GRAY, DARKGRAY, GOLD, ORANGE, PINK, MAROON, GREEN, LIME, DARKGREEN, SKYBLUE, BLUE,
    DARKBLUE, VIOLET, PURPLE, BEIGE, BROWN, DARKBROWN, BLACK, MAGENTA,
];

#[macroquad::main("My game")]
async fn main() {
    let mut highscore = 0u32;
    let mut score = 0u32;
    let mut gameover = false;
    let mut squares = vec![];
    let mut bullets = vec![];
    let mut last_shot_time = get_time();
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.,
        y: screen_height() / 2.,
        color: YELLOW,
        collided: false,
    };

    rand::srand(miniquad::date::now() as u64);

    loop {
        let delta_time = get_frame_time();
        let circle_movement = MOVEMENT_SPEED * delta_time;

        if !gameover {
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(size / 2.0, screen_width() - size / 2.0),
                    y: -size,
                    color: *COLOR_LIST.choose().unwrap(),
                    collided: false,
                });
            }

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

            if last_shot_time + BULLET_COOLDOWN < get_time()
                && bullets.len() < MAX_BULLETS
                && is_key_pressed(KeyCode::Space)
            {
                bullets.push(Shape {
                    size: 5.0,
                    speed: circle.speed * 2.0,
                    x: circle.x,
                    y: circle.y,
                    color: RED,
                    collided: false,
                });
                last_shot_time = get_time();
            }

            circle.x = clamp(circle.x, 0.0, screen_width());
            circle.y = clamp(circle.y, 0.0, screen_height());

            for square in &mut squares {
                square.y += square.speed * delta_time;
            }
            squares.retain(|square| square.y < screen_height() + square.size);

            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
            bullets.retain(|bullet| bullet.y > -bullet.size / 2.0);

            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                        score += square.size.round() as u32;
                        highscore = highscore.max(score);
                    }
                }
            }

            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);

            gameover = squares.iter().any(|square| circle.collides_with(square));
        }

        if gameover && is_key_pressed(KeyCode::Enter) {
            squares.clear();
            bullets.clear();
            circle.x = screen_width() / 2.0;
            circle.y = screen_height() / 2.0;
            gameover = false;
            score = 0;
        }

        /* draw stuff from here on down */
        clear_background(DARKPURPLE);

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }
        draw_circle(circle.x, circle.y, 16.0, circle.color);

        if gameover {
            const TEXT: &str = "GAME OVER!";
            let text_dimensions = measure_text(TEXT, None, 50, 1.0);
            draw_text(
                TEXT,
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0,
                50.0,
                RED,
            );
        }

        draw_text(format!("Score: {score}").as_str(), 10., 35., 25., WHITE);
        let highscore_string = format!("High score: {highscore}");
        let highscore_text = highscore_string.as_str();
        let td = measure_text(highscore_text, None, 25, 1.0);
        draw_text(
            highscore_text,
            screen_width() - td.width - 10.0,
            35.,
            25.,
            WHITE,
        );

        next_frame().await
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
    collided: bool,
}

impl Shape {
    fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.size / 2.0,
            y: self.y - self.size / 2.0,
            w: self.size,
            h: self.size,
        }
    }
}
