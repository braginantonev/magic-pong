use bevy::prelude::*;

use super::PPos;

#[derive(Event)]
pub struct IncreaseScoreEvent(pub PPos);

#[derive(Resource, Default)]
pub struct PlayersScore {
    right_player: u32,
    left_player: u32
}

impl PlayersScore {
    pub fn get(&self, player_pos: PPos) -> u32 {
        match player_pos {
            PPos::Right => self.right_player,
            PPos::Left => self.left_player
        }
    }

    pub fn add_point_to_left(&mut self) {
        self.left_player += 1;
    }

    pub fn add_point_to_right(&mut self) {
        self.right_player += 1;
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<IncreaseScoreEvent>()
            .init_resource::<PlayersScore>();
    }
}