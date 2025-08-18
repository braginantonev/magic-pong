pub mod spawn;
mod restart;

use bevy::prelude::*;

const BALL_SIZE: Vec2 = vec2(75.0, 75.0);
pub const MAX_START_SPEED_X: f32 = 500.0;
pub const MAX_START_SPEED_Y: f32 = 200.0;

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
            restart::RestartPlugin
        ));
    }
}