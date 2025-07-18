use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{assets::GameAssets, GameState};
use super::{Ball, MAX_START_SPEED_X, MAX_START_SPEED_Y, BALL_SIZE};

const MIN_VELOCITY_COEFFICIENT: f32 = 0.7;

pub fn gen_random_velocity_coef() -> f32 {
    let mut rng = rand::rng();
    let coef = rng.random_range(MIN_VELOCITY_COEFFICIENT..=1.0);
    [coef, -coef][rng.random_range(0..=1)]
}

fn spawn_ball(mut commands: Commands, asset: Res<GameAssets>) {
    commands.spawn((
        Ball,
        Sprite {
            image: asset.ball.clone(),
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE.x / 2.0),
        Restitution::coefficient(1.0),
        GravityScale(0.0),
        Friction::coefficient(0.0),
        Ccd::enabled(),
        ActiveEvents::COLLISION_EVENTS,
        CollisionGroups::default(),
        Velocity::zero()
    ));
}

fn add_velocity(mut velocity: Query<&mut Velocity, With<Ball>>) {
    *velocity.single_mut().unwrap() = Velocity::linear(vec2(gen_random_velocity_coef() * MAX_START_SPEED_X, gen_random_velocity_coef() * MAX_START_SPEED_Y));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(crate::GameState::SpawnMainEntities), spawn_ball)
            .add_systems(OnEnter(GameState::InGame), add_velocity);
    }
}