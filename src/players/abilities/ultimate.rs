use bevy::prelude::*;

use crate::{
    GameState,
    players::{ Player, PPos, score::IncreaseScoreEvent }
};

fn update_ultimate_progress(
    mut score_event: EventReader<IncreaseScoreEvent>,
    q_players: Query<&mut Player>
) {
    if score_event.is_empty() {
        println!("score event in ult is empty");
        return
    }

    let mut ppos = PPos::Left;
    for ev in score_event.read() {
        ppos = ev.0;
    }

    for mut player in q_players {
        if player.position == ppos {
            player.increase_ultimate_progression();
            break;
        }
    }
}

fn use_ultimate_by_input(
    input: Res<ButtonInput<KeyCode>>,
    q_players: Query<&mut Player>
) {
    for mut player in q_players {
        match player.position {
            PPos::Left => if input.just_pressed(KeyCode::KeyD) { player.use_ultimate(); },
            PPos::Right => if input.just_pressed(KeyCode::ArrowRight) { player.use_ultimate(); }
        }
    }
}

pub struct UltimatePlugin;

impl Plugin for UltimatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Restart), update_ultimate_progress)
            .add_systems(Update, use_ultimate_by_input.run_if(in_state(GameState::InGame)).after(update_ultimate_progress));
    }
}