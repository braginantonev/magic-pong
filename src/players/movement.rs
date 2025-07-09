use bevy::prelude::*;
use mathx::Math;

use super::*;

const Y_LIMIT: f32 = crate::WINDOW_SIZE.y / 2.0 - PLAYER_SIZE.y / 2.0;

fn move_by_input_r(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<Player>, With<Right>)>,
    
) {
    let mut transform = query.single_mut().unwrap();

    if input.pressed(KeyCode::ArrowUp) {
        transform.translation.y = Math::clamp(transform.translation.y + SPEED * time.delta_secs(), -Y_LIMIT, Y_LIMIT);
    } else if input.pressed(KeyCode::ArrowDown) {
        transform.translation.y = Math::clamp(transform.translation.y - SPEED * time.delta_secs(), -Y_LIMIT, Y_LIMIT);
    }
}

fn move_by_input_l(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<Player>, With<Left>)>
) {
    let mut transform = query.single_mut().unwrap();

    if input.pressed(KeyCode::KeyW) {
        transform.translation.y = Math::clamp(transform.translation.y + SPEED * time.delta_secs(), -Y_LIMIT, Y_LIMIT);
    } else if input.pressed(KeyCode::KeyS) {
        transform.translation.y = Math::clamp(transform.translation.y - SPEED * time.delta_secs(), -Y_LIMIT, Y_LIMIT);
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_by_input_r, move_by_input_l).run_if(in_state(crate::GameState::InGame)));
    }
}