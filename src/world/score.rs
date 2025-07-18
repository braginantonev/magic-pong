use bevy::prelude::*;

use crate::GameState;

pub const UPDATE_SCORE_DURATION: f32 = 2.0;

#[derive(Resource)]
struct UpdateScoreTimer(Timer);

fn start_timer(mut timer: ResMut<UpdateScoreTimer>) {
    timer.0.reset();
}

fn update_score(
    time: Res<Time>,
    mut timer: ResMut<UpdateScoreTimer>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::Restart);
    }
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(UpdateScoreTimer(Timer::from_seconds(UPDATE_SCORE_DURATION, TimerMode::Once)))
            .add_systems(OnEnter(GameState::UpdateScore), start_timer)
            .add_systems(Update, update_score.run_if(in_state(GameState::UpdateScore)));
    }
}