// https://dev.to/qongzi/bevy-minesweeper-part-2-1hi5
// https://github.com/leonidv/bevy-minesweeper-tutorial/commit/ab2518b46abeccc76a790ff6602667236ccf3d97#diff-2e9d962a08321605940b5a657135052fbcef87b5e360662bb527c96d9a615542R11

use bevy::{prelude::*, window::WindowResolution};
use bevy::log::LogPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::{BoardOptions, BoardPosition, BoardSize};
use board_plugin::resources::TileSize::Fixed;
// #[cfg(feature = "debug")]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    NewGame,
    #[default] 
    InGame,
    Pause,
    EndGame
}

fn main() {
    let mut app = App::new();
    let mut primary_window = Window::default();
    primary_window.resolution = WindowResolution::new(850.0, 850.0);
    primary_window.title = "Mine Sweeper!".to_string();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(primary_window),
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
                })
            .set(LogPlugin {
                ..Default::default()
            }),
    );
    app.insert_resource(BoardOptions {
        map_size: BoardSize { columns: 20, rows: 20 },
        bomb_count: 40,
        position: BoardPosition::Centered { offset: Vec3::ZERO },
        tile_padding: 1.5,
        tile_size: Fixed(35.0),
        safe_start: true,
        game_state: AppState::InGame,
        pause_state: AppState::Pause
    });
    
    app.init_state::<AppState>();
    // app.insert_state(AppState::NewGame);
    // app.insert_state(AppState::InGame);
    // app.insert_state(AppState::Pause);
    // app.insert_state(AppState::EndGame);
    
    app.add_plugins(BoardPlugin {
        game_state: AppState::InGame,
        pause_state: AppState::Pause,
    });
    // #[cfg(feature = "debug")]
    // .add_plugins(WorldInspectorPlugin::new())
    app.add_systems(Startup, setup_camera);
    app.run();

    // Debug hierarchy inspector
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
