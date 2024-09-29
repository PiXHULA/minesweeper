// https://dev.to/qongzi/bevy-minesweeper-part-2-1hi5
// https://github.com/leonidv/bevy-minesweeper-tutorial/commit/ab2518b46abeccc76a790ff6602667236ccf3d97#diff-2e9d962a08321605940b5a657135052fbcef87b5e360662bb527c96d9a615542R11

use bevy::prelude::*;
use bevy::window::WindowResolution;
use board_plugin::BoardPlugin;
use board_plugin::resources::BoardOptions;
use board_plugin::resources::TileSize::Fixed;
// #[cfg(feature = "debug")]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mine Sweeper!".to_string(),
                resolution: WindowResolution::new(800.0, 1000.0),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 1.5,
            tile_size: Fixed(30.0),
            safe_start: true,
            ..Default::default()
        })
        .add_plugins(BoardPlugin)
        // #[cfg(feature = "debug")]
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_camera)
        .run();

    // Debug hierarchy inspector
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            // transform: Transform::from_xyz(100.0, 200.0, 0.0),
            ..default()
        },
    ));
}
