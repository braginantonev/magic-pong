use bevy::prelude::*;

use crate::GameState;

#[derive(Resource)]
pub struct RestartTimer(pub Timer);

const RESTART_DURATION: f32 = 1.0;

fn start_timer(mut timer: ResMut<RestartTimer>) {
    timer.0.reset();
}

fn restart_game(
    time: Res<Time>,
    mut timer: ResMut<RestartTimer>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::InGame);
    }
}

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(RestartTimer(Timer::from_seconds(super::score::UPDATE_SCORE_DURATION + RESTART_DURATION, TimerMode::Once)))
            .add_systems(OnEnter(GameState::Restart), start_timer)
            .add_systems(Update, restart_game.run_if(in_state(GameState::Restart)));
    }
}