pub mod components;
pub mod resources;
mod bounds;
mod systems;
mod events;


use std::collections::HashMap;
use bevy::color::palettes::tailwind;
use crate::components::{Coordinates, Uncover};
use crate::resources::tile::Tile;
use crate::resources::{BoardPosition, TileSize};
use bevy::prelude::*;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::Board;
use crate::bounds::Bounds2;
use crate::events::TileTriggerEvent;

pub struct BoardPlugin;

impl BoardPlugin {
    //System to generate the complete board
    fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        asset_server: Res<AssetServer>,
    ) {
        //Load assets
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");

        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };

        let tile_size = match options.tile_size {
            TileSize::Fixed(size) => size,
            TileSize::Adaptive { .. } => panic!(
                "Not supported in this commit due to WindowDescriptor is not available as resource"
            ),
        };


        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);

        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        #[cfg(feature = "debug")]
        info!("board_size: {}", &board_size);

        // We define the board anchor position (bottom left)
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3 {
                    x: -(board_size.x / 2.0),
                    y: -(board_size.y / 2.0),
                    z: 0.0,
                } + offset
            }
            BoardPosition::Custom(p) => p,
        };

        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        info!("info: {}", tile_map.console_output());

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());
        
        let mut safe_start: Option<Entity> = None;
        
        commands
            .spawn((
                Name::new("Board"),
                SpatialBundle {
                    transform: Transform::from_translation(board_position),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::from(tailwind::GRAY_50),
                            // color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(
                            board_size.x / 2.,
                            board_size.y / 2.,
                            0.,
                        ),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    // Color::srgba(0.2, 0.2, 0.2, 1.0),
                    Color::from(tailwind::GRAY_400),
                    bomb_image,
                    font,
                    Color::from(tailwind::GRAY_900),
                    &mut covered_tiles,
                    &mut safe_start,
                );
                //color: Color::srgba(0.2, 0.2, 0.2, 1.0),
            });
        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }

        commands.insert_resource(Board {
            tile_map: tile_map.clone(),
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
        });
    }
    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        tile_size: f32,
        tile_padding: f32,
        background_color: Color,
        bomb_image: Handle<Image>,
        font: Handle<Font>,
        covered_tile_color: Color,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        let tile_real_size = tile_size - tile_padding;
        let sprites_size =  Some(Vec2::splat(tile_real_size));

        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    coord_x: x as u16,
                    coord_y: y as u16,
                };

                #[cfg(feature = "debug")]
                info!("Spawn tile {:?} at {:?}", tile, coordinates);

                let mut commands = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: background_color,
                        custom_size: Some(Vec2::splat(tile_size - tile_padding)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * tile_size) + (tile_size / 2.0),
                        (y as f32 * tile_size) + (tile_size / 2.0),
                        1.0,
                    ),
                    ..Default::default()
                });

                commands
                    .insert(Name::new(format!("Tile: ({}, {})", x, y)))
                    .insert(coordinates);

                commands.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle {
                            sprite: Sprite {
                                custom_size: sprites_size,
                                color: covered_tile_color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 2.0),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });

                match tile {
                    Tile::Bomb => {
                        commands.insert(components::Bomb);
                        commands.with_children(|parent| {
                            parent.spawn(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: sprites_size,
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                texture: bomb_image.clone(),
                                ..Default::default()
                            });
                        });
                    }
                    Tile::BombNeighbor(bombs_count) => {
                        commands.insert(components::BombNeighbor { count: *bombs_count });
                        commands.with_children(|parent| {
                            parent.spawn(Self::bomb_count_text_bundle(
                                *bombs_count,
                                font.clone(),
                                tile_size - tile_padding,
                            ));
                        });
                    }
                    Tile::Empty => (),
                }
            }
        }
    }

    fn bomb_count_text_bundle(count: u8, font: Handle<Font>, font_size: f32) -> Text2dBundle {
        //Retrieve the text and the correct color
        let color =
            match count {
                1 => Color::WHITE,
                2 => Color::srgba(0.25, 0.9, 0.25, 1.0),  //limegreen
                3 => Color::srgba(1., 1., 0., 1.0),       //yellow
                4 => Color::srgba(1., 0.5, 0., 1.0),      //orange
                5 => Color::srgba(1., 0.2, 0.15, 1.0),    //tomato
                _ => Color::srgba(0.5, 0., 0.5, 1.0),     //purple
            };

        let style = TextStyle {
            font,
            font_size,
            color,
        };

        let bomb_count_text = Text::from_section(count.to_string(), style).with_justify(JustifyText::Center);

        Text2dBundle {
            text: bomb_count_text,
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        }
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board)
           .add_systems(Update, systems::input::input_handling)
           .add_systems(Update, systems::uncover::trigger_event_handler)
           .add_systems(Update, systems::uncover::uncover_tiles)
           .add_event::<TileTriggerEvent>();
        info!("Loaded Board Plugin");
        // #[cfg(feature = "debug")]
        // {
        //     // registering custom component to be able to edit it in inspector
        //     app.register_inspectable::<Coordinates>();
        //     app.register_inspectable::<BombNeighbor>();
        //     app.register_inspectable::<Bomb>();
        //     app.register_inspectable::<Uncover>();
        // }
    }
}
