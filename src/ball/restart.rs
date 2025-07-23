use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use super::Ball;
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

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Restart), (stop_ball, return_to_spawn_position));
    }
}