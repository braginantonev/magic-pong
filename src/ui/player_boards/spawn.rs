use bevy::prelude::*;

use crate::{ assets::GameAssets, GameState, WINDOW_SIZE };
use super::{ PlayerBoard, PPos, BOARD_SIZE };

const LAYER_Z: f32 = 0.0;

const RIGHT_BOARD_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0, 0.0, LAYER_Z);
const LEFT_BOARD_POSITION: Vec3 = vec3(-RIGHT_BOARD_POSITION.x, 0.0, LAYER_Z);

fn spawn_boards(
    assets: Res<GameAssets>,
    mut commands: Commands
) {
    // Left player board
    commands.spawn((
        PlayerBoard(PPos::Left),
        Sprite {
            image: assets.player_board.clone(),
            custom_size: Some(BOARD_SIZE),
            ..default()
        },
        Transform::from_translation(LEFT_BOARD_POSITION)
    ));

    // Right player board
    commands.spawn((
        PlayerBoard(PPos::Right),
        Sprite {
            image: assets.player_board.clone(),
            custom_size: Some(BOARD_SIZE),
            ..default()
        },
        Transform::from_translation(RIGHT_BOARD_POSITION)
    ));
}

pub struct PlayerBoardsSpawnPlugin;

impl Plugin for PlayerBoardsSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnMainEntities), spawn_boards);
    }
}