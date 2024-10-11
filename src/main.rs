use bevy::{prelude::*, window::WindowResolution};
use bevy::color::palettes::tailwind;
use bevy::log::LogPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::{BoardAssets, BoardOptions, BoardSize, SpriteMaterial};
use board_plugin::resources::TileSize::Fixed;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default] 
    Setup,
    NewGame,
    InGame,
    Pause,
    EndGame
}

fn transition_to_in_game(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InGame);
}

fn main() {
    let mut primary_window = Window::default();
    primary_window.resolution = WindowResolution::new(850.0, 850.0);
    primary_window.title = "Mine Sweeper!".to_string();
    App::new()
    .add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(primary_window),
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
                })
            .set(LogPlugin {
                ..Default::default()
            }),
    )
    // https://bevyengine.org/learn/migration-guides/0-13-to-0-14/#onenter-state-schedules-now-run-before-startup-schedules
    .init_state::<AppState>()
    .add_systems(OnEnter(AppState::Setup), (setup_camera, setup_board))
    .add_systems(Update, transition_to_in_game.run_if(in_state(AppState::Setup)))
    .add_plugins(BoardPlugin {
        game_state: AppState::InGame,
        pause_state: AppState::Pause,
    })    
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial::color(Color::from(tailwind::STONE_50)),
        tile_material: SpriteMaterial::color(Color::from(tailwind::STONE_400)),
        covered_tile_material: SpriteMaterial::color(Color::from(tailwind::STONE_800)),
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial::texture(&asset_server.load("sprites/flag.png")),
        bomb_material: SpriteMaterial::texture(&asset_server.load("sprites/bomb.png")),
        menu_font: asset_server.load("fonts/neuropol_x_rg.otf"),
    });
    commands.insert_resource(BoardOptions {
        map_size: BoardSize { columns: 20, rows: 20 },
        bomb_count: 60,
        position: board_plugin::resources::BoardPosition::Centered { offset: Vec3::ZERO },
        tile_size: Fixed(35.0),
        tile_padding: 3.0,
        safe_start: true,
        game_state: AppState::InGame,
        pause_state: AppState::Pause,
    })
}
