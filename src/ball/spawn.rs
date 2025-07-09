use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{assets::GameAssets, WINDOW_SIZE};

const BALL_SIZE: Vec2 = vec2(75.0, 75.0);

fn get_force_point() -> Vec2 {
    let mut rng = rand::rng();
    vec2(rng.random_range(-0.8..=0.8) * WINDOW_SIZE.x, rng.random_range(-0.8..=0.8) * WINDOW_SIZE.y)
}

fn spawn_ball(mut commands: Commands, asset: Res<GameAssets>) {
    commands.spawn((
        super::Ball,
        Sprite {
            image: asset.ball.clone(),
            custom_size: Some(BALL_SIZE),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE.x / 2.0),
        Restitution::coefficient(1.0),
        GravityScale(0.0),
        //Todo: Доделать эту херню надо
        //Velocity::linear(Velocity::linear_velocity_at_point(&Velocity::zero(), get_force_point(), vec2(0.0, 0.0)))
    ));
}

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::GameState::InGame), spawn_ball);
    }
}