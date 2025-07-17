use bevy::prelude::*;

use crate::GameState;

const SPAWN_ENTITIES_DURATION: f32 = 1.0;

#[derive(Resource)]
struct SpawnMainEntitiesTimer(Timer);

fn start_game(
    time: Res<Time>,
    mut timer: ResMut<SpawnMainEntitiesTimer>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        next_state.set(GameState::InGame);
    }
}

pub struct SpawnEntitiesPlugin;

impl Plugin for SpawnEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SpawnMainEntitiesTimer(Timer::from_seconds(SPAWN_ENTITIES_DURATION, TimerMode::Once)))
            .add_systems(Update, start_game.run_if(in_state(GameState::SpawnMainEntities)));
    }
}