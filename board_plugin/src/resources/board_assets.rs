use bevy::color::palettes::tailwind;
use bevy::prelude::{Color, Font, Handle, Image, Resource};
// use bevy::render::*;

#[derive(Debug, Clone, Default)]
pub struct SpriteMaterial {
    pub color: Color,
    pub texture: Handle<Image>,
}

impl SpriteMaterial {
    pub fn color(color: Color) -> SpriteMaterial {
        SpriteMaterial {
            color,
            ..Default::default()
            // texture: texture::TRANSPARENT_IMAGE_HANDLE,
        }
    }

    pub fn texture(texture: &Handle<Image>) -> SpriteMaterial {
        SpriteMaterial {
            texture: texture.clone(),
            ..Default::default()
        }
    }
}

/// Assets for the board. Must be used as a resource.
///
/// Use the loader for partial setup
#[derive(Debug, Clone, Resource)]
pub struct BoardAssets {
    pub label: String,
    pub board_material: SpriteMaterial,
    pub tile_material: SpriteMaterial,
    pub covered_tile_material: SpriteMaterial,
    pub bomb_counter_font: Handle<Font>,
    pub bomb_counter_colors: Vec<Color>,
    pub flag_material: SpriteMaterial,
    pub bomb_material: SpriteMaterial,
    pub menu_font: Handle<Font>,
}

impl BoardAssets {
    ///Default bomb counter color set
    pub fn default_colors() -> Vec<Color> {
        vec![
           Color::from(tailwind::STONE_50),
           Color::from(tailwind::LIME_400),
           Color::from(tailwind::YELLOW_400),
           Color::from(tailwind::ORANGE_400),
           Color::from(tailwind::RED_400),
           Color::from(tailwind::INDIGO_400),
        ]
    }

    //Safely retrieves the color matching a bomb counter
    pub fn bomb_counter_color(&self, counter: u8) -> Color {
        let counter = counter.saturating_sub(1) as usize;
        match self.bomb_counter_colors.get(counter) {
            Some(color) => *color,
            None => *self.bomb_counter_colors.last().unwrap()
        }
    }
}
