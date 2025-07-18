pub mod score;
pub mod spawn;
mod restart;
mod movement;

use bevy::prelude::*;

const SPEED: f32 = 250.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, crate::WINDOW_SIZE.y / 3.5);

enum Position {
    Right,
    Left
}

#[derive(Component)]
pub struct Player(Position);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
            movement::MovementPlugin,
            score::ScorePlugin,
            restart::RestartPlugin,
        ));
    }
}