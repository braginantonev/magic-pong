use bevy::prelude::*;

use super::{
    Player, Left, Right,
    spawn::{
        RIGHT_PLAYER_START_POSITION,
        LEFT_PLAYER_START_POSITION
    }
};

use crate::GameState;

fn return_to_start_position(
    mut q_r_player: Query<&mut Transform, (With<Player>, With<Right>)>,
    mut q_l_player: Query<&mut Transform, (With<Player>, With<Left>)>,
) {
    q_r_player.single_mut().unwrap().translation = RIGHT_PLAYER_START_POSITION;
    q_l_player.single_mut().unwrap().translation = LEFT_PLAYER_START_POSITION;
}

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Restart), return_to_start_position);
    }
}