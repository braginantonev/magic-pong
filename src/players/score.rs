use bevy::prelude::*;

#[derive(Event)]
pub struct IncreaseScoreEvent(pub super::PPos);

#[derive(Resource, Default)]
pub struct PlayersScore {
    right_player: u32,
    left_player: u32
}

impl PlayersScore {
    pub fn left_score(&self) -> u32 {
        self.left_player
    }

    pub fn right_score(&self) -> u32 {
        self.right_player
    }

    pub fn add_point_to_left(&mut self) {
        self.left_player += 1;
        //Todo: add update ui score label
    }

    pub fn add_point_to_right(&mut self) {
        self.right_player += 1;
        //Todo: add update ui score label
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