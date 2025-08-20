use bevy::prelude::*;

use crate::{
    players::{ PPos, Player },
    GameState
};

use super::{ 
    UseAbilityEvent,
    SkillsList,
    Stager,
    StageTimer,
    AbilitiesList,
};

pub struct SkillPlugin;

impl Plugin for SkillPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<UseAbilityEvent<SkillsList>>()
            .add_systems(Update, (tick_skill_timer, use_skill_by_input).run_if(in_state(GameState::InGame)))
            .add_systems(Update, start_skill_stages.run_if(in_state(GameState::InGame)).run_if(on_event::<UseAbilityEvent<SkillsList>>));
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
    mut ability_event: EventWriter<UseAbilityEvent<SkillsList>>
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

fn start_skill_stages(
    mut commands: Commands,
    mut ability_event: EventReader<UseAbilityEvent<SkillsList>>,
) {
    let stager = commands.spawn(Stager).id();
    
    for ev in ability_event.read() {
        commands.entity(stager).insert(match ev.get_ability() {
            SkillsList::Revert => StageTimer::new(ev.pos, AbilitiesList::Skill(SkillsList::Revert)),
        });
    }
}