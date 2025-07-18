mod spawn;
mod update;

use bevy::prelude::*;

const TEXT_SIZE: f32 = 48.0;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Left;

#[derive(Component)]
struct Right;

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