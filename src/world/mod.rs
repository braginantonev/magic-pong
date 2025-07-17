pub mod restart;
pub mod score;
mod spawn_entities;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                restart::RestartPlugin,
                score::ScorePlugin,
                spawn_entities::SpawnEntitiesPlugin
            ));
    }
}