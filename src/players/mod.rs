pub mod score;
pub mod spawn;
mod restart;
mod movement;

use bevy::prelude::*;

const SPEED: f32 = 250.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, crate::WINDOW_SIZE.y / 3.5);

pub enum PPos {
    Right,
    Left
}

#[derive(Component)]
pub struct Player {
    position: PPos,
    ultimate_progress: u8,
    is_available_ultimate: bool
}

impl Player {
    fn new(pos: PPos) -> Self {
        Self { position: pos, ultimate_progress: 0, is_available_ultimate: false }
    }

    fn ultimate_progress(&self) -> u8 {
        self.ultimate_progress
    }

    fn add_point_to_ultimate(&mut self) {
        if self.ultimate_progress + 1 > 4 {
            self.is_available_ultimate = true;
            self.ultimate_progress = 0;
            return 
        }

        self.ultimate_progress += 1;
    }

    fn ultimate(&mut self) {
        if !self.is_available_ultimate {
            error!("use ultimate, while he not available");
            return
        }

        self.is_available_ultimate = false;
    }
}

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