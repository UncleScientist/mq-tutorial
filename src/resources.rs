use macroquad::{
    audio::{load_sound, Sound},
    color::{BLACK, WHITE},
    file::load_file,
    math::RectOffset,
    prelude::{collections::storage, coroutines::start_coroutine},
    texture::{build_textures_atlas, load_image, load_texture, FilterMode, Texture2D},
    time::get_time,
    ui::{root_ui, Skin, StyleBuilder},
    window::{clear_background, next_frame},
};

pub struct Resources {
    pub ship_texture: Texture2D,
    pub bullet_texture: Texture2D,
    pub explosion_texture: Texture2D,
    pub enemy_small_texture: Texture2D,
    pub enemy_medium_texture: Texture2D,
    pub enemy_big_texture: Texture2D,
    pub theme_music: Sound,
    pub sound_explosion: Sound,
    pub sound_laser: Sound,
    pub ui_skin: Skin,
}

impl Resources {
    pub async fn new() -> Result<Self, macroquad::Error> {
        let ship_texture = load_texture_from_file("ship.png").await?;
        let bullet_texture = load_texture_from_file("laser-bolts.png").await?;
        let explosion_texture = load_texture_from_file("explosion.png").await?;
        let enemy_small_texture = load_texture_from_file("enemy-small.png").await?;
        let enemy_medium_texture = load_texture_from_file("enemy-medium.png").await?;
        let enemy_big_texture = load_texture_from_file("enemy-big.png").await?;
        build_textures_atlas();

        let theme_music = load_sound("8bit-spaceshooter.ogg").await.unwrap();
        let sound_explosion = load_sound("explosion.wav").await.unwrap();
        let sound_laser = load_sound("laser.wav").await.unwrap();

        let window_background = load_image("window_background.png").await?;
        let button_background = load_image("button_background.png").await?;
        let button_clicked_background = load_image("button_clicked_background.png").await?;
        let font = load_file("atari_games.ttf").await?;

        let window_style = root_ui()
            .style_builder()
            .background(window_background)
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(button_background)
            .background_clicked(button_clicked_background)
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .set_font(&font, 64)?
            .build();

        let label_style = root_ui().style_builder().set_font(&font, 28)?.build();

        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };
        Ok(Resources {
            ship_texture,
            bullet_texture,
            explosion_texture,
            enemy_small_texture,
            enemy_medium_texture,
            enemy_big_texture,
            theme_music,
            sound_explosion,
            sound_laser,
            ui_skin,
        })
    }

    pub async fn load() -> Result<(), macroquad::Error> {
        let resources_loading = start_coroutine(async move {
            let resources = Resources::new().await.unwrap();
            storage::store(resources);
        });

        while !resources_loading.is_done() {
            clear_background(BLACK);
            crate::draw_text_centered(
                &format!(
                    "Loading resources {}",
                    ".".repeat(((get_time() * 2.0) as usize) % 4)
                ),
                0.0,
            );
            next_frame().await;
        }

        Ok(())
    }
}

async fn load_texture_from_file<P: std::fmt::Debug + AsRef<str>>(
    path: P,
) -> Result<Texture2D, macroquad::Error> {
    let texture = load_texture(path.as_ref()).await?;
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}

trait FontSetter: Sized {
    fn set_font(self, font: &[u8], size: u16) -> Result<Self, macroquad::Error>;
}

impl FontSetter for StyleBuilder {
    fn set_font(self, font: &[u8], size: u16) -> Result<Self, macroquad::Error> {
        Ok(self.font(font)?.text_color(WHITE).font_size(size))
    }
}
