pub mod score;
mod spawn;

use bevy::prelude::*;

const BOARD_SIZE: Vec2 = vec2(100.0, crate::WINDOW_SIZE.y);

enum PPos {
    Right,
    Left
}

#[derive(Component)]
struct PlayerBoard(PPos);

pub struct PlayerBoardsPlugin;

impl Plugin for PlayerBoardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            score::ScorePlugin,
            spawn::PlayerBoardsSpawnPlugin
        ));
    }
}