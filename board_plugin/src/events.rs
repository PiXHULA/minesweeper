use bevy::prelude::Event;

use crate::components::Coordinates;

#[derive(Debug, Copy, Clone, Event)]
pub struct TileTriggerEvent{
    pub coordinates: Coordinates
}

#[derive(Debug, Copy, Clone, Event)]
pub struct TileMarkEvent{
    pub coordinates: Coordinates
}


#[derive(Debug, Copy, Clone, Event)]
pub struct BoardCompletedEvent;

#[derive(Debug, Copy, Clone, Event)]
pub struct BombExplosionEvent;

