use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;
use crate::WINDOW_SIZE;

use super::*;

const X_OFFSET: f32 = 35.0;

fn spawn_players(mut commands: Commands) {
    // Left player
    commands.spawn((
        Player,
        Left,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(-WINDOW_SIZE.x / 2.0 + X_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
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
        Transform::from_xyz(WINDOW_SIZE.x / 2.0 - X_OFFSET, 0.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_players);
    }
}