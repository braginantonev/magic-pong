use bevy::prelude::*;

use crate::{ GameState, players::score::PlayersScore };
use super::{ Score, Left, Right };

fn update_score(
    mut q_r_score: Query<&mut Text2d, (With<Score>, With<Right>, Without<Left>)>,
    mut q_l_score: Query<&mut Text2d, (With<Score>, With<Left>, Without<Right>)>,
    players_score: Res<PlayersScore>
) {
    match q_l_score.single_mut() {
        Ok(mut r) => { r.0 = players_score.left_score().to_string() },
        Err(_) => {
            error!("left score not found!");
            return
        }
    };

    match q_r_score.single_mut() {
        Ok(mut r) => { r.0 = players_score.right_score().to_string() },
        Err(_) => {
            error!("right score not found!");
            return
        }
    };
}

pub struct ScoreUpdatePlugin;

impl Plugin for ScoreUpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::UpdateScore), update_score);
    }
}