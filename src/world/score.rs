use bevy::prelude::*;

use crate::GameState;

pub const UPDATE_SCORE_DURATION: f32 = 2.0;

fn update_score(
    mut score_event: EventReader<crate::players::score::IncreaseScoreEvent>,
    mut next_state: ResMut<NextState<GameState>>
) {
    for _ in score_event.read() {
        next_state.set(GameState::Restart)
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, update_score);
    }
}