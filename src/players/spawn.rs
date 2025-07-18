use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ GameState, WINDOW_SIZE };

use super::{ Player, PLAYER_SIZE, Position };

const X_OFFSET: f32 = 35.0 + crate::walls::X_OFFSET;
pub const LEFT_PLAYER_START_POSITION: Vec3 = vec3(-WINDOW_SIZE.x / 2.0 + X_OFFSET, 0.0, 0.0);
pub const RIGHT_PLAYER_START_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - X_OFFSET, 0.0, 0.0);

fn spawn_players(mut commands: Commands) {
    // Left player
    commands.spawn((
        Player(Position::Left),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(LEFT_PLAYER_START_POSITION.x, LEFT_PLAYER_START_POSITION.y, LEFT_PLAYER_START_POSITION.z),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
    ));

    // Right player
    commands.spawn((
        Player(Position::Right),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(super::PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(RIGHT_PLAYER_START_POSITION.x, RIGHT_PLAYER_START_POSITION.y, RIGHT_PLAYER_START_POSITION.z),
        RigidBody::Fixed,
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SpawnMainEntities), spawn_players);
    }
}