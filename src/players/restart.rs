use bevy::prelude::*;

use super::{
    Player, PPos,
    spawn::{
        RIGHT_PLAYER_START_POSITION,
        LEFT_PLAYER_START_POSITION
    }
};

use crate::GameState;

fn return_to_start_position(
    q_players: Query<(&mut Transform, &Player)>,
) {
    for (mut transform, player) in q_players {
        match player.position {
            PPos::Left => transform.translation = LEFT_PLAYER_START_POSITION,
            PPos::Right => transform.translation = RIGHT_PLAYER_START_POSITION
        }
    }
}

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Restart), return_to_start_position);
    }
}