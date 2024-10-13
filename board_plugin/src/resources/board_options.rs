use bevy::{prelude::Vec3, ecs::system::Resource};
use bevy::math::Vec2;
use bevy::state::state::FreelyMutableState;
use serde::{Deserialize, Serialize};

//Tile size options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    // Fixed tile size
    Fixed(f32),
    // Window adaptative tile size
    Adaptive { min: f32, max: f32 },
}

//Board position customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardPosition {
    // Centered board
    Centered { offset: Vec3 },
    // Custom poisition
    Custom(Vec3),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSize {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions<T: FreelyMutableState> {
    // Tile map size
    pub map_size: BoardSize,
    // bomb count
    pub bomb_count: u16,
    // board world position
    pub position: BoardPosition,
    // tile world size
    pub tile_size: TileSize,
    // padding between tiles,
    pub tile_padding: f32,
    //makes the first click safe by shifting the mine 
    // to the first empty square on the top row, 
    // starting from the left corner.
    pub safe_start: bool,
    //State with active game
    pub game_state: T,
    //State with paused game
    pub pause_state: T,
    //State with end game
    pub game_over_state: T,
}

impl<T: FreelyMutableState> BoardOptions<T> {
    pub fn tile_size_px(&self) -> f32 {
        match self.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => panic!(
                "Not supported in this commit due to WindowDescriptor is not available as resource"
            )
        }
    }

    pub fn board_size(&self) -> Vec2 {
        let tile_size_pixels = self.tile_size_px();
        Vec2::new(
            self.map_size.columns as f32 * tile_size_pixels,
            self.map_size.rows as f32 * tile_size_pixels
        )
    }

    // We define the board anchor position (bottom left)
    pub fn board_position_px(&self, z_layer: f32) -> Vec3 {
        let board_size = self.board_size();
        
        match self.position {
            BoardPosition::Centered { offset } => {
                Vec3 {
                    x: -(board_size.x / 2.0),
                    y: -(board_size.y / 2.0),
                    z: z_layer,
                } + offset
            }
            BoardPosition::Custom(p) => p,
        }
    }
}

impl Default for TileSize {
    fn default() -> Self {
        Self::Adaptive {
            min: 10.0,
            max: 50.0,
        }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Default::default(),
        }
    }
}

// impl <T : States> Default for BoardOptions<T> {
//     fn default() -> Self {
//         Self {
//             map_size: (15, 15),
//             bomb_count: 30,
//             position: Default::default(),
//             tile_size: Default::default(),
//             tile_padding: 0.0,
//             safe_start: false,
//             game_state: Default::default(),
//         }
//     }
// }