pub mod components;
pub mod resources;
mod bounds;
mod systems;
mod events;


use std::collections::HashMap;
use std::default::{Default};
use bevy::color::palettes::tailwind;
use crate::components::{Coordinates, Uncover, PauseCover};
use crate::resources::tile::Tile;
use crate::resources::{BoardPosition, TileSize};
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use resources::tile_map::TileMap;
use resources::BoardOptions;
use resources::Board;
use crate::bounds::Bounds2;
use crate::events::TileTriggerEvent;

/// White box
const BACKGROUND_Z: f32 = 0.0;
/// Tiles - boxed above background
const TILE_Z: f32 = 1.0;
/// Count of neighors bombs, bomb, etc.
const TILE_INFO_Z: f32 = 2.0;
/// Box above tile which is still not uncover by player
const TILE_COVER_Z: f32 = 3.0;
/// Pause box
const PAUSE_COVER_Z: f32 = 100.0;

pub struct BoardPlugin<T>
where
    T: FreelyMutableState,
{
    pub game_state: T,
    pub pause_state: T,
}

impl<T: FreelyMutableState> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(self.game_state.clone()), Self::create_board)
            .add_systems(OnExit(self.game_state.clone()), Self::on_exit_log)
            .add_systems(
                Update,
                (
                    systems::input::input_handling,
                    systems::uncover::trigger_event_handler,
                    systems::uncover::uncover_tiles,
                    Self::recreate_board,
                    Self::pause,
                ).run_if(in_state(self.game_state.clone())))
            .add_systems(
                Update,
                (
                    Self::unpause
                ).run_if(in_state(self.pause_state.clone())))
            .add_event::<TileTriggerEvent>();

        info!("Loaded Board Plugin");
    }
}

impl<T: FreelyMutableState> BoardPlugin<T> {
    //System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOptions<T>>,
        board_option: Option<Res<Board>>,
        asset_server: Res<AssetServer>,
    ) {
        if board_option.is_some() {
            return;
        }

        //Load assets
        let font: Handle<Font> = asset_server.load("fonts/pixeled.ttf");
        let bomb_image: Handle<Image> = asset_server.load("sprites/bomb.png");

        let options = board_options.clone();

        let tile_size = options.tile_size_px();

        let mut tile_map = TileMap::empty(options.map_size.columns, options.map_size.rows);

        let board_size = options.board_size();

        #[cfg(feature = "debug")]
        info!("board_size: {}", &board_size);

        let board_position = options.board_position_px(BACKGROUND_Z);

        tile_map.set_bombs(options.bomb_count);
        #[cfg(feature = "debug")]
        info!("info: {}", tile_map.console_output());

        let mut covered_tiles =
            HashMap::with_capacity((tile_map.width() * tile_map.height()).into());

        let mut safe_start: Option<Entity> = None;

        let board_entity = commands
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
                            board_size.x / 2.0,
                            board_size.y / 2.0,
                            BACKGROUND_Z,
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
            })
            .id();

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
            entity: board_entity,
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
        let sprites_size = Some(Vec2::splat(tile_real_size));

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
                        TILE_Z,
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
                            transform: Transform::from_xyz(0.0, 0.0, TILE_COVER_Z),
                            //TODO: FORTSÄTTH HÄR
                            // https://github.com/leonidv/bevy-minesweeper-tutorial/commit/6229aca4282ce473f38bcb3193c40a2bd33e520a#diff-411126a4d292d9529da7e23553aa1015d93ac7014e9565bdb7de1ef1e78cc37b
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
                                transform: Transform::from_xyz(0.0, 0.0, TILE_INFO_Z),
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

        //TODO: Check how to center text
        let bomb_count_text = Text::from_section(count.to_string(), style).with_justify(JustifyText::Center);

        Text2dBundle {
            text: bomb_count_text,
            transform: Transform::from_xyz(0.0, 0.0, TILE_INFO_Z),
            ..Default::default()
        }
    }

    fn recreate_board(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        board: Res<Board>,
        asset_server: Res<AssetServer>,
        board_options: Res<BoardOptions<T>>,
    ) {
        if keys.just_released(KeyCode::KeyG) {
            info!("G is released");
            commands.entity(board.entity).despawn_recursive();
            BoardPlugin::create_board(commands, board_options, None, asset_server)
        }
    }

    fn pause(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        mut next_state: ResMut<NextState<T>>,
        board_options: Res<BoardOptions<T>>,
        asset_server: Res<AssetServer>,
    ) {
        if keys.just_released(KeyCode::KeyP) {
            next_state.set(board_options.pause_state.clone());

            let font: Handle<Font> = asset_server.load("fonts/neuropol_x_rg.otf");
            let text_style = TextStyle {
                font,
                font_size: board_options.tile_size_px(),
                color: Color::from(tailwind::YELLOW_200),
            };
            let text = Text::from_section("Paused! Press P to continue", text_style)
                .with_justify(JustifyText::Center);

            let board_size = board_options.board_size();
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::from(tailwind::TEAL_300),
                        custom_size: Some(board_size),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, PAUSE_COVER_Z),
                    ..Default::default()
                })
                .insert(Name::new("Pause cover"))
                .insert(PauseCover)
                .with_children(|parent| {
                    parent.spawn(Text2dBundle {
                        text,
                        transform: Transform::from_xyz(0.0, 0.0, PAUSE_COVER_Z + 1.0),
                        ..Default::default()
                    });
                });
        }
    }

    fn unpause(
        mut commands: Commands,
        keys: Res<ButtonInput<KeyCode>>,
        mut next_state: ResMut<NextState<T>>,
        board_options: Res<BoardOptions<T>>,
        pause_cover_query: Query<Entity, With<PauseCover>>,
    ) {
        if keys.just_released(KeyCode::KeyP) {
            let x: Entity = pause_cover_query.single();
            commands.entity(x).despawn_recursive();
            next_state.set(board_options.game_state.clone())
        }
    }

    fn on_exit_log() {
        info!("exit from state")
    }
}

