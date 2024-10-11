use std::collections::HashMap;
use bevy::prelude::{error, Entity, Resource, Vec2, Window};
use crate::bounds::Bounds2;
use crate::components::Coordinates;
use crate::resources::tile_map::TileMap;

#[derive(Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
    pub entity: Entity,
    pub marked_tiles: Vec<Coordinates>,
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
        if self.marked_tiles.contains(coordinates) {
            None
        } else {
            self.covered_tiles.get(coordinates)
        }
    }

    /// We try to uncover a tile, returning the entity
    pub fn try_uncover_tile(&mut self, coordinates: &Coordinates) -> Option<Entity> {
        if self.marked_tiles.contains(coordinates) {
            self.unmark_tile(coordinates);
        }
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
    
    //// Try to mark or unmark a tile, returning the entity and if the tile is marked
    pub fn try_toggle_mark(&mut self, coordinates: &Coordinates) -> Option<(Entity, bool)> {
        let entity = *self.covered_tiles.get(coordinates)?;
        let mark = if self.marked_tiles.contains(coordinates) {
            self.unmark_tile(coordinates)?;
            false
        } else {
            self.marked_tiles.push(*coordinates);
            true
        };
        Some((entity, mark))
    }
    
    ///Remove the coords from marked_tiles
    fn unmark_tile(&mut self, coordinates: &Coordinates) -> Option<Coordinates> {
        let pos = match self.marked_tiles.iter().position(|a| a == coordinates) {
            None => {
                error!("Failed to unmark tile at {}", coordinates);
                return None;
            }
            Some(p) => p,
        };
        Some(self.marked_tiles.remove(pos))
    }
    
    ///Check if board is completed
    pub fn is_completed(&self) -> bool {
        self.tile_map.bomb_count() as usize == self.covered_tiles.len()
    }
}