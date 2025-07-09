mod spawn;

use bevy::prelude::*;

#[derive(Component)]
struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
        ));
    }
}