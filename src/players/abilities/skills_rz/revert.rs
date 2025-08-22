use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    ball::Ball, 
    players::{
        PPos, 
        Player,
        abilities::{ 
            AbilitiesList,
            SkillsList,
            StageEntered,
            abilities_db::AbilitiesInfo,
        },
    },
    GameState,
};

pub struct SkillRevertPlugin;

impl Plugin for SkillRevertPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Skill1sRevertEvent>()
            .add_systems(Update, sk_revert_1s.run_if(in_state(GameState::InGame)).run_if(on_event::<Skill1sRevertEvent>))
            .add_systems(Update, check_revert_stages.run_if(in_state(GameState::InGame)).run_if(on_event::<StageEntered>))
        ;
    }
}

//* Revert stages events */

#[derive(Event)]
struct Skill1sRevertEvent(PPos);

// Кто это читает, знайте, что этот код полное дерьмище, конкретно из-за этой части можно побить автора, то есть меня
// Никогда так не делайте пожалуйста
fn check_revert_stages(
    abilities_info: Res<AbilitiesInfo>,
    mut skill_1s_ev_writer: EventWriter<Skill1sRevertEvent>,
    mut stage_entered_ev: EventReader<StageEntered>,
) {
    for ev in stage_entered_ev.read() {
        if ev.ability != AbilitiesList::Skill(SkillsList::Revert) {
            return
        }

        match abilities_info.get_stage_counter(ev.player, ev.ability).get_current_stage() {
            0 => skill_1s_ev_writer.write(Skill1sRevertEvent(ev.player)),
            _ => continue,
        };
    }
}

fn sk_revert_1s(
    mut asr_event: EventReader<Skill1sRevertEvent>,
    mut q_ball: Query<&mut Velocity, With<Ball>>,
    q_players: Query<(&Transform, &Player)>
) {
    let mut velocity = q_ball.single_mut().unwrap();

    let mut ev_pos = PPos::Left;

    for ev in asr_event.read() {
        ev_pos = ev.0;
    }

    for (transform, player) in q_players {
        if player.get_pos() == ev_pos {
            continue
        }

        let k = if transform.translation.y >= 0.0 { -1.0 } else { 1.0 };
        velocity.linvel = vec2(-velocity.linvel.x, velocity.linvel.y * k);
    }
}