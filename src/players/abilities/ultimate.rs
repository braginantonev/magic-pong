use bevy::prelude::*;

use crate::{
    players::
    { 
        score::IncreaseScoreEvent, 
        PPos, 
        PState,
        Player,
    },
    GameState
};

use super::{
    UseAbilityEvent,
    Stager,
    StageTimer,
    UltimatesList,
    AbilitiesList,
};

pub struct UltimatePlugin;

impl Plugin for UltimatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UseAbilityEvent<UltimatesList>>()
            .add_systems(OnEnter(GameState::Restart), update_ultimate_progress)
            .add_systems(Update, use_ultimate_by_input.run_if(in_state(GameState::InGame)))
            .add_systems(Update, start_ultimate_stages.run_if(in_state(GameState::InGame)).run_if(on_event::<UseAbilityEvent<UltimatesList>>));
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

fn start_ultimate_stages(
    mut commands: Commands,
    mut ability_event: EventReader<UseAbilityEvent<UltimatesList>>,
    q_player: Query<&mut Player>,
) {
    let stager = commands.spawn(Stager).id();
    let mut pos = PPos::Left;
    
    for ev in ability_event.read() {
        commands.entity(stager).insert(match ev.get_ability() {
            UltimatesList::Debug1 => StageTimer::new(ev.pos, AbilitiesList::Ultimate(UltimatesList::Debug1)),
            UltimatesList::Debug2 => todo!(),
        });
        pos = ev.pos;
    }

    for mut player in q_player {
        if player.get_pos() != pos {
            player.state = PState::Silence;
        }
    }
}