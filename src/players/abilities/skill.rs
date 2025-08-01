use bevy::prelude::*;

use crate::{
    players::{ PPos, Player },
    GameState
};

use super::{ UseAbilityEvent, Skills };

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UseAbilityEvent<Skills>>()
            .add_systems(Update, (tick_skill_timer, use_skill_by_input).run_if(in_state(GameState::InGame)));
    }
}

fn tick_skill_timer(
    time: Res<Time>,
    q_players: Query<&mut Player>
) {
    for mut player in q_players {
        if player.skill_is_available() {
            continue
        }

        player.skill_timer.tick(time.delta());
    }
}

fn use_skill_by_input(
    q_players: Query<&mut Player>,
    input: Res<ButtonInput<KeyCode>>,
    mut ability_event: EventWriter<UseAbilityEvent<Skills>>
) {
    for mut player in q_players {
        match player.get_pos() {
            PPos::Right => if input.just_pressed(KeyCode::ArrowLeft) {
                if let Some(skill) = player.use_skill() {
                    ability_event.write(UseAbilityEvent { pos: player.get_pos(), ability: skill });
                }
            },
            PPos::Left => if input.just_pressed(KeyCode::KeyA) {
                if let Some(skill) = player.use_skill() {
                    ability_event.write(UseAbilityEvent { pos: player.get_pos(), ability: skill });
                }
            }
        }
    }
}