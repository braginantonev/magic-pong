use bevy::prelude::*;

use crate::WINDOW_SIZE;
use crate::GameState;

const X_OFFSET: f32 = 25.0;

#[derive(Component, Debug)]
enum Position {
    Right,
    Left
}

impl Position {
    pub fn get_cords(&self) -> Vec3 {
        match self {
            Self::Right => vec3(WINDOW_SIZE.x / 2.0 - X_OFFSET, 0.0, 0.0),
            Self::Left => vec3(WINDOW_SIZE.x / -2.0 + X_OFFSET, 0.0, 0.0)
        }
    }
}

#[derive(Component)]
struct Player;

fn spawn_players(mut commands: Commands) {
    let right_cords = Position::Right.get_cords();
    let left_cords = Position::Left.get_cords();

    commands.spawn((
        Player,
        Position::Left,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(super::PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(left_cords.x, left_cords.y, left_cords.z)
    ));

    commands.spawn((
        Player,
        Position::Right,
        Sprite {
            color: Color::WHITE,
            custom_size: Some(super::PLAYER_SIZE),
            ..default()
        },
        Transform::from_xyz(right_cords.x, right_cords.y, right_cords.z)
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_players);
    }
}