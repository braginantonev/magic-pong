use bevy::prelude::*;
use mathx::Math;

use super::{ Player, PPos, PState, PLAYER_SIZE, SPEED };

const Y_LIMIT: f32 = crate::WINDOW_SIZE.y / 2.0 - PLAYER_SIZE.y / 2.0;

fn move_players(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    q_players: Query<(&mut Transform, &Player)>
) {
    let arrows_coef = if input.pressed(KeyCode::ArrowUp) {
        1.0
    } else if input.pressed(KeyCode::ArrowDown) {
        -1.0
    } else {
        0.0
    };

    let ws_coef = if input.pressed(KeyCode::KeyW) {
        1.0
    } else if input.pressed(KeyCode::KeyS) {
        -1.0
    } else {
        0.0
    };

    for (mut transform, player) in q_players {
        if player.state == PState::Stun {
            continue
        }

        match player.position {
            PPos::Left => transform.translation.y = Math::clamp(transform.translation.y + SPEED * time.delta_secs() * ws_coef, -Y_LIMIT, Y_LIMIT),
            PPos::Right => transform.translation.y = Math::clamp(transform.translation.y + SPEED * time.delta_secs() * arrows_coef, -Y_LIMIT, Y_LIMIT)
        } 
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_players.run_if(in_state(crate::GameState::InGame)));
    }
}