use bevy::prelude::*;

use crate::{
    players::{ score::IncreaseScoreEvent, PPos, Player },
    GameState
};

use super::{
    UseAbilityEvent,
    UltimatesList,
};

pub struct UltimatePlugin;

impl Plugin for UltimatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UseAbilityEvent<UltimatesList>>()
            .add_systems(OnEnter(GameState::Restart), update_ultimate_progress)
            .add_systems(Update, use_ultimate_by_input.run_if(in_state(GameState::InGame)).after(update_ultimate_progress));
    }
}

//* -- Systems -- */

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
    mut ability_event: EventWriter<UseAbilityEvent<UltimatesList>>,
    q_players: Query<&mut Player>
) {
    for mut player in q_players {
        match player.position {
            PPos::Left => if input.just_pressed(KeyCode::KeyD) {
                if let Some(used_ult) = player.use_ultimate() {
                    ability_event.write(UseAbilityEvent{ pos: player.position, ability: used_ult });
                }  
            },
            PPos::Right => if input.just_pressed(KeyCode::ArrowRight) {
                if let Some(used_ult) = player.use_ultimate() {
                    ability_event.write(UseAbilityEvent{ pos: player.position, ability: used_ult });
                }  
            }
        }
    }
}
