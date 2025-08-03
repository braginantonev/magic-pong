use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ 
    Ability, AbilityStage,
    First
};

use crate::{ball::Ball, players::{PPos, Player}, GameState};

pub struct SkillRevertPlugin;

impl Plugin for SkillRevertPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AbilityStage<Revert, First>>()
            .add_systems(Update, s_rev_first_stage.run_if(in_state(GameState::InGame)).run_if(on_event::<AbilityStage<Revert, First>>));
    }
}

pub struct Revert;

impl Ability for Revert {
    fn to_str(&self) -> String {
        "Revert".to_string()
    }
}

fn s_rev_first_stage(
    mut asr_event: EventReader<AbilityStage<Revert, First>>,
    mut q_ball: Query<&mut Velocity, With<Ball>>,
    q_players: Query<(&Transform, &Player)>
) {
    let mut velocity = q_ball.single_mut().unwrap();

    let mut ev_pos = PPos::Left;

    for ev in asr_event.read() {
        ev_pos = ev.ppos;
    }

    for (transform, player) in q_players {
        if player.get_pos() == ev_pos {
            continue
        }

        let k = if transform.translation.y >= 0.0 { -1.0 } else { 1.0 };
        velocity.linvel = vec2(-velocity.linvel.x, velocity.linvel.y * k);
    }
}