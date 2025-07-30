pub mod score;
mod spawn;
mod placeholder;

use bevy::prelude::*;
use crate::players::PPos;

pub const BOARD_SIZE: Vec2 = vec2(100.0, crate::WINDOW_SIZE.y);

#[derive(Component)]
struct PlayerBoard;

pub struct PlayerBoardsPlugin;

impl Plugin for PlayerBoardsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            score::ScorePlugin,
            placeholder::PlaceholderPlugin,
            spawn::PlayerBoardsSpawnPlugin
        ));
    }
}