use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use crate::events::TileMarkEvent;
use crate::resources::{Board, BoardAssets};
use crate::TILE_Z;

pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    mut tile_mark_event_reader: EventReader<TileMarkEvent>,
    query: Query<&Children>,
) {
    for tile_mark_event in tile_mark_event_reader.read() {
        if let Some((entity, mark)) = board.try_toggle_mark(&tile_mark_event.coordinates) {
            if mark {
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: board_assets.flag_material.texture.clone(),
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(board.tile_size)),
                                color: board_assets.tile_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0., TILE_Z),
                            ..Default::default()
                        })
                        .insert(Name::new("Flag"));
                });
            } else {
                let children = match query.get(entity) {
                    Ok(c) => c,
                    Err(e) => {
                        #[cfg(feature = "debug")]
                        error!("Failied to retrieve flag entity components: {}", e);
                        continue;
                    }
                };
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}