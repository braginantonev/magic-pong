mod players;
mod ball;
mod walls;
mod assets;
mod ui;
mod world;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;
use bevy_asset_loader::prelude::*;

const WINDOW_SIZE: Vec2 = vec2(1050.0, 500.0);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetsLoading,
    SpawnMainEntities,
    InGame,
    UpdateScore,
    Restart
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Magic pong".into(),
                        resolution: WindowResolution::new(WINDOW_SIZE.x, WINDOW_SIZE.y),
                        ..default()
                    }),
                    ..default()
                }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            }
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading).continue_to_state(GameState::SpawnMainEntities),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetsLoading).load_collection::<assets::GameAssets>(),
        )
        .add_systems(OnEnter(GameState::SpawnMainEntities), setup)
        .add_plugins((
            players::PlayersPlugin,
            ball::BallPlugin,
            walls::WallsPlugin,
            world::WorldPlugin,
            ui::UIPlugin
        ))
        .run();
}
