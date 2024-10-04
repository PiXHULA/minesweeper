use bevy::log::{error, info};
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, EventReader, Parent, Query, Res, ResMut, With};
use crate::components::{Bomb, BombNeighbor, Coordinates, Uncover};
use crate::events::TileTriggerEvent;
use crate::resources::Board;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event_reader: EventReader<TileTriggerEvent>
) {
    for trigger_event in tile_trigger_event_reader.read() {
        #[cfg(feature = "debug")]
        info!("Tile trigger event handler {:?}", trigger_event);
        if let Some(entity) = board.tile_to_uncover(&trigger_event.coordinates) {
            #[cfg(feature = "debug")]
            info!("Insert uncover to {:?}", entity);
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
) {
    // Iterate through tile covers to uncover
    for (entity, parent) in children.iter() {
        // Destroy the tile cover entity
       commands.entity(entity).despawn_recursive();

        let (coordinates, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        match board.try_uncover_tile(coordinates) {
            None => {
                #[cfg(feature = "debug")]
                info!("Tried to uncover an already uncovered tile")
            },
            Some(_e) => {
                #[cfg(feature = "debug")]
                info!("Uncovered tile {} (entity: {:?})",coordinates, e)
            },
        }

        if bomb.is_some() {
            #[cfg(feature = "debug")]
            info!("Boom!");
            //TODO: Add explosion event
        }
        // if the tile is empty (no bomb near tile)..
        else if bomb_counter.is_none() {
            // ..We propagate the uncovering by adding the 'Uncover'
            // which will then be removed next frame
            for entity in board.adjacent_covered_tiles(*coordinates) {
                commands.entity(entity).insert(Uncover);
            };
        }

    }
}