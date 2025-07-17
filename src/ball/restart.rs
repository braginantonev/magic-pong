use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::{spawn::gen_random_velocity_coef, MAX_START_SPEED_X, MAX_START_SPEED_Y, Ball};
use crate::GameState;

fn return_to_spawn_position(
    mut velocity: Query<&mut Velocity, With<Ball>>,
    mut transform: Query<&mut Transform, With<Ball>>
) {
    *velocity.single_mut().unwrap() = Velocity::zero();
    transform.single_mut().unwrap().translation = Vec3::ZERO;
}

fn stop_ball(mut velocity: Query<&mut Velocity, With<Ball>>) {
    *velocity.single_mut().unwrap() = Velocity::zero();
}

fn reforce(mut velocity: Query<&mut Velocity, With<Ball>>) {
    *velocity.single_mut().unwrap() = Velocity::linear(vec2(gen_random_velocity_coef() * MAX_START_SPEED_X, gen_random_velocity_coef() * MAX_START_SPEED_Y))
}

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::UpdateScore), stop_ball)
            .add_systems(OnEnter(GameState::Restart), return_to_spawn_position)
            .add_systems(OnExit(GameState::Restart), reforce);
    }
}