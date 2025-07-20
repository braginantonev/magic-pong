mod spawn;
mod update;

use bevy::prelude::*;
use super::PPos;

const TEXT_SIZE: f32 = 48.0;

#[derive(Component)]
struct Score {
    ppos: PPos,
    need_update: bool
}

impl Score {
    fn new(player_position: PPos) -> Self {
        Score { ppos: player_position, need_update: false }
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                spawn::ScoreSpawnPlugin,
                update::ScoreUpdatePlugin
            ));
    }
}