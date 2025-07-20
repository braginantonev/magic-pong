pub mod score;
mod spawn;

use bevy::prelude::*;

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
        ));
    }
}