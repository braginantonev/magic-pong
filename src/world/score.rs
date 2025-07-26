use bevy::prelude::*;

use crate::GameState;

pub const UPDATE_SCORE_DURATION: f32 = 2.0;

fn update_score(
    mut score_event: EventReader<crate::players::score::IncreaseScoreEvent>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for _ in score_event.par_read() {
        next_state.set(GameState::Restart)
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostUpdate, update_score.run_if(in_state(GameState::InGame)));
    }
}