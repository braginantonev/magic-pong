use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;
use crate::WINDOW_SIZE;

use super::*;

const X_OFFSET: f32 = 35.0 + crate::walls::X_OFFSET;
const LEFT_PLAYER_START_POSITION: Vec3 = vec3(-WINDOW_SIZE.x / 2.0 + X_OFFSET, 0.0, 0.0);
const RIGHT_PLAYER_START_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - X_OFFSET, 0.0, 0.0);

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
        Transform::from_xyz(LEFT_PLAYER_START_POSITION.x, LEFT_PLAYER_START_POSITION.y, LEFT_PLAYER_START_POSITION.z),
        RigidBody::Fixed,
        Friction::coefficient(0.0),
        Restitution::coefficient(1.0),
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
        Transform::from_xyz(RIGHT_PLAYER_START_POSITION.x, RIGHT_PLAYER_START_POSITION.y, RIGHT_PLAYER_START_POSITION.z),
        RigidBody::Fixed,
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
    ));
}

fn return_to_start_position(
    mut q_r_player: Query<&mut Transform, (With<Player>, With<Right>)>,
    mut q_l_player: Query<&mut Transform, (With<Player>, With<Left>)>,
) {
    q_r_player.single_mut().unwrap().translation = RIGHT_PLAYER_START_POSITION;
    q_l_player.single_mut().unwrap().translation = LEFT_PLAYER_START_POSITION;
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_players)
            .add_systems(OnEnter(GameState::Restart), return_to_start_position);
    }
}