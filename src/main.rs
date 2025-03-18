mod shader;

use macroquad::experimental::animation::{AnimatedSprite, Animation};
use macroquad_particles::{self as particles, ColorCurve, Emitter};

use macroquad::prelude::*;
use rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 200.0;

const MAX_BULLETS: usize = 7;
const BULLET_COOLDOWN: f64 = 0.25;
const SHIP_FLAME_COUNT: usize = 1;

const COLOR_LIST: [Color; 20] = [
    LIGHTGRAY, GRAY, DARKGRAY, GOLD, ORANGE, PINK, MAROON, GREEN, LIME, DARKGREEN, SKYBLUE, BLUE,
    DARKBLUE, VIOLET, PURPLE, BEIGE, BROWN, DARKBROWN, MAGENTA, DARKPURPLE,
];

#[macroquad::main("My game")]
async fn main() {
    let mut high_score = 0u32;
    let mut score = 0u32;
    let mut game_state = GameState::MainMenu;
    let mut got_high_score = false;
    let mut squares = vec![];
    let mut bullets = vec![];
    let mut explosions = vec![];
    let mut last_shot_time = get_time();
    let mut flames = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.,
        y: screen_height() / 2.,
        color: YELLOW,
        collided: false,
    };

    set_pc_assets_folder("assets");

    let ship_texture = load_texture("ship.png").await.expect("Loading ship png");
    ship_texture.set_filter(FilterMode::Nearest);
    let bullet_texture = load_texture("laser-bolts.png")
        .await
        .expect("Loading bullet png");
    bullet_texture.set_filter(FilterMode::Nearest);
    build_textures_atlas();

    let mut bullet_sprite = AnimatedSprite::new(
        16,
        16,
        &[
            Animation {
                name: "bullet".into(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "bolt".into(),
                row: 1,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );
    bullet_sprite.set_animation(1);

    let mut ship_sprite = AnimatedSprite::new(
        16,
        24,
        &[
            Animation {
                name: "idle".into(),
                row: 0,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "left".into(),
                row: 2,
                frames: 2,
                fps: 12,
            },
            Animation {
                name: "right".into(),
                row: 4,
                frames: 2,
                fps: 12,
            },
        ],
        true,
    );

    rand::srand(miniquad::date::now() as u64);

    let mut direction_modifier: f32 = 0.0;
    let render_target = render_target(320, 150); // width, height
    let material = load_material(
        ShaderSource::Glsl {
            vertex: shader::VERTEX_SHADER,
            fragment: shader::FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        let delta_time = get_frame_time();
        let circle_movement = MOVEMENT_SPEED * delta_time;

        // If we're actively playing, calculate the next frame
        if matches!(game_state, GameState::Playing) {
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

            ship_sprite.set_animation(0);
            if is_key_down(KeyCode::Right) {
                circle.x += circle_movement;
                direction_modifier += 0.05 * delta_time;
                ship_sprite.set_animation(2);
            }
            if is_key_down(KeyCode::Left) {
                circle.x -= circle_movement;
                direction_modifier -= 0.05 * delta_time;
                ship_sprite.set_animation(1);
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
                    size: 32.0,
                    speed: circle.speed * 2.0,
                    x: circle.x,
                    y: circle.y - 24.0,
                    color: RED,
                    collided: false,
                });
                last_shot_time = get_time();
            }

            if is_key_pressed(KeyCode::Escape) {
                game_state = GameState::Paused;
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

            ship_sprite.update();
            bullet_sprite.update();

            for square in squares.iter_mut() {
                for bullet in bullets.iter_mut() {
                    if bullet.collides_with(square) {
                        bullet.collided = true;
                        square.collided = true;
                        score += square.size.round() as u32;
                        if score > high_score {
                            got_high_score = true;
                            high_score = score;
                        }
                        explosions.push((
                            Emitter::new(particle_explosion(
                                square.size.round() as u32 * 2,
                                ExplosionDirection::Circular,
                            )),
                            vec2(square.x, square.y),
                        ));
                    }
                }
            }

            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
            explosions.retain(|(explosion, _)| explosion.config.emitting);

            if squares.iter().any(|square| circle.collides_with(square)) {
                game_state = GameState::GameOver;
            }
        }

        /* draw everything */
        clear_background(BLACK);

        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();

        for square in &squares {
            draw_rectangle(
                square.x - square.size / 2.0,
                square.y - square.size / 2.0,
                square.size,
                square.size,
                square.color,
            );
        }

        let bullet_frame = bullet_sprite.frame();
        for bullet in &bullets {
            draw_texture_ex(
                &bullet_texture,
                bullet.x - bullet.size / 2.0,
                bullet.y - bullet.size / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bullet.size, bullet.size)),
                    source: Some(bullet_frame.source_rect),
                    ..Default::default()
                },
            );
            // draw_circle(bullet.x, bullet.y, bullet.size / 2.0, RED);
        }

        if matches!(game_state, GameState::Playing) || !flames.is_empty() {
            let ship_pos = vec2(circle.x, circle.y);
            if matches!(game_state, GameState::Playing) && flames.len() < SHIP_FLAME_COUNT {
                flames.push(Emitter::new(particle_explosion(
                    200,
                    ExplosionDirection::Below,
                )));
            }
            for flame in &mut flames {
                flame.config.one_shot = !matches!(game_state, GameState::Playing);
                flame.draw(ship_pos);
            }
            flames.retain(|flame| flame.config.emitting);
        }

        let ship_frame = ship_sprite.frame();
        draw_texture_ex(
            &ship_texture,
            circle.x - ship_frame.dest_size.x,
            circle.y - ship_frame.dest_size.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(ship_frame.dest_size * 2.0),
                source: Some(ship_frame.source_rect),
                ..Default::default()
            },
        );
        // draw_circle(circle.x, circle.y, 16.0, circle.color);

        for (explosion, coords) in &mut explosions {
            explosion.draw(*coords);
        }

        draw_text(format!("Score: {score}").as_str(), 10., 35., 25., WHITE);
        let highscore_string = format!("High score: {high_score}");
        let highscore_text = highscore_string.as_str();
        let td = measure_text(highscore_text, None, 25, 1.0);
        draw_text(
            highscore_text,
            screen_width() - td.width - 10.0,
            35.,
            25.,
            WHITE,
        );

        /* check and handle key input in non-playing states */
        match game_state {
            GameState::MainMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Enter) {
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    flames.clear();
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    game_state = GameState::Playing;
                    got_high_score = false;
                    score = 0;
                }

                draw_text_centered("Circles and Squares", -5.0);
                draw_text_centered("Press ENTER", 0.0);
            }
            GameState::Paused => {
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::Playing;
                }
                draw_text_centered("Paused", 0.0);
            }
            GameState::GameOver => {
                if is_key_pressed(KeyCode::Enter) {
                    game_state = GameState::MainMenu;
                }
                draw_text_centered("GAME OVER!", 0.0);
                if got_high_score {
                    draw_text_centered("NEW HIGH SCORE!", 1.0);
                }
            }

            GameState::Playing => {
                // Do nothing: all the work was done up above
            }
        }

        next_frame().await
    }
}

fn draw_text_centered(text: &str, line: f32) {
    const TEXT_HEIGHT: f32 = 50.0;
    const BORDER: f32 = 4.0;

    let td = measure_text(text, None, TEXT_HEIGHT as u16, 1.0);
    let ypos = screen_height() / 2.0 + TEXT_HEIGHT * line;
    let x = screen_width() / 2.0 - td.width / 2.0;
    let baseline = ypos;
    let y = baseline - td.offset_y;
    draw_rectangle(x, y - BORDER, td.width, td.height + 2. * BORDER, BLACK);
    draw_text(text, x, baseline, TEXT_HEIGHT, RED);
}

enum ExplosionDirection {
    Circular,
    Below,
}

fn particle_explosion(amount: u32, dir: ExplosionDirection) -> particles::EmitterConfig {
    particles::EmitterConfig {
        amount,
        local_coords: false,
        one_shot: matches!(dir, ExplosionDirection::Circular),
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction: match dir {
            ExplosionDirection::Circular => vec2(0., -1.),
            ExplosionDirection::Below => vec2(0., 1.),
        },
        initial_direction_spread: match dir {
            ExplosionDirection::Circular => 2.0 * std::f32::consts::PI,
            ExplosionDirection::Below => std::f32::consts::PI / 2.0,
        },
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: YELLOW,
        },
        ..Default::default()
    }
}

enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
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
