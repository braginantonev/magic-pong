use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::WINDOW_SIZE;

use super::{Wall, X_OFFSET, HORIZONTAL_WALL_SIZE, VERTICAL_WALL_SIZE};

fn spawn_walls(mut commands: Commands) {
    // Horizontal up wall
    commands.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(HORIZONTAL_WALL_SIZE.x / 2.0, HORIZONTAL_WALL_SIZE.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(HORIZONTAL_WALL_SIZE),
            ..default()
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Transform::from_xyz(0.0, WINDOW_SIZE.y / 2.0 - HORIZONTAL_WALL_SIZE.y, 0.0),
    ));

    // Horizontal down wall
    commands.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(HORIZONTAL_WALL_SIZE.x / 2.0, HORIZONTAL_WALL_SIZE.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(HORIZONTAL_WALL_SIZE),
            ..default()
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Transform::from_xyz(0.0, -WINDOW_SIZE.y / 2.0 + HORIZONTAL_WALL_SIZE.y, 0.0),
    ));

    // Vertical right wall
    commands.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(VERTICAL_WALL_SIZE.x / 2.0, VERTICAL_WALL_SIZE.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(VERTICAL_WALL_SIZE),
            ..default()
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Transform::from_xyz(WINDOW_SIZE.x / 2.0 - VERTICAL_WALL_SIZE.x - X_OFFSET, 0.0, 0.0),
    ));

    // Vertical left wall
    commands.spawn((
        Wall,
        RigidBody::Fixed,
        Collider::cuboid(VERTICAL_WALL_SIZE.x / 2.0, VERTICAL_WALL_SIZE.y / 2.0),
        Sprite {
            color: Color::WHITE,
            custom_size: Some(VERTICAL_WALL_SIZE),
            ..default()
        },
        Restitution::coefficient(1.0),
        Friction::coefficient(0.0),
        Transform::from_xyz(-WINDOW_SIZE.x / 2.0 + VERTICAL_WALL_SIZE.x + X_OFFSET, 0.0, 0.0),
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::InGame), spawn_walls);
    }
}