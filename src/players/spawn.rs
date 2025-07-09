use bevy::prelude::*;

use crate::GameState;
use crate::WINDOW_SIZE;

use super::{Player, Left, Right};

const X_OFFSET: f32 = 25.0;

fn spawn_players(mut commands: Commands) {
    // Left player
    commands.spawn((
        Player,
        Left,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(super::PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(-WINDOW_SIZE.x / 2.0 + X_OFFSET, 0.0, 0.0)
    ));

    // Right player
    commands.spawn((
        Player,
        Right,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(super::PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(WINDOW_SIZE.x / 2.0 - X_OFFSET, 0.0, 0.0)
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_players);
    }
}