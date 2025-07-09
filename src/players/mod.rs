mod movement;
mod spawn;

use bevy::prelude::{Vec2, Plugin, App};

const SPEED: f32 = 200.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, crate::WINDOW_SIZE.y / 3.5);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
        ));
    }
}