use std::collections::HashMap;
use bevy::prelude::{Entity, Resource, Vec2, Window};
use crate::bounds::Bounds2;
use crate::components::Coordinates;
use crate::resources::tile_map::TileMap;

#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
}

impl Board {
    //Translate mouse position to board coordinates
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        //Window to world space
        let window_size = Vec2 {
            x: window.width(),
            y: window.height(),
        };
        let position_at_board = position - (window_size / 2.0);

        // Bounds check
        if !self.bounds.in_bounds(position_at_board) {
            return None;
        }

        // World space to board space
        let coordinates = position_at_board - self.bounds.position;
        Some(Coordinates {
            coord_x: (coordinates.x / self.tile_size) as u16,
            coord_y: self.tile_map.height() - 1 - (coordinates.y / self.tile_size) as u16,
        })
    }

    /// Retrivies a covered tile entity
    pub fn tile_to_uncover(&self, coordinates: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coordinates)
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coordinates)
    }

    /// We retrieve the adjacent covered tile entities of `coordinates`
    pub fn adjacent_covered_tiles(&self, coordinates: Coordinates) -> Vec<Entity> {
        self
            .tile_map
            .safe_square_at(coordinates)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}