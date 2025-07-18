mod spawn;
mod update;

use bevy::prelude::*;

const TEXT_SIZE: f32 = 48.0;

enum Type {
    Right,
    Left
}

#[derive(Component)]
struct Score {
    r#type: Type,
    need_update: bool
}

impl Score {
    fn new(_type: Type) -> Self {
        Score { r#type: _type, need_update: false }
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