use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    ball::{
        spawn::gen_random_velocity_coef,
        Ball, MAX_START_SPEED_X, MAX_START_SPEED_Y
    }, 
    ge_utils::animation::{
        GameEntityAnimation,
        SEVec3,
    }, 
    players::{
        abilities::{
            abilities_db::AbilitiesInfo, AbilitiesList, StageEntered, UltimatesList
        },
        PPos,
        PState,
        Player
    }, 
    GameState
};

pub struct Debug1Plugin;

impl Plugin for Debug1Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Ult1sDebug1Event>()
            .add_event::<Ult2sDebug1Event>()
            .add_event::<Ult3sDebug1Event>()
            .add_systems(Update, check_debug1_stages.run_if(in_state(GameState::InGame)).run_if(on_event::<StageEntered>))
            .add_systems(Update, ul_db1_1s.run_if(in_state(GameState::InGame)).run_if(on_event::<Ult1sDebug1Event>))
            .add_systems(Update, ul_db1_2s.run_if(in_state(GameState::InGame)).run_if(on_event::<Ult2sDebug1Event>))
            .add_systems(Update, ul_db1_3s.run_if(in_state(GameState::InGame)).run_if(on_event::<Ult3sDebug1Event>))
        ;
    }
}

//* Revert stages events */

#[derive(Event)]
struct Ult1sDebug1Event;

#[derive(Event)]
struct Ult2sDebug1Event;

/// Last stage (PPos needed)
#[derive(Event)]
struct Ult3sDebug1Event(PPos);

// Кто это читает, знайте, что этот код полное дерьмище, конкретно из-за этой части можно побить автора, то есть меня
// Никогда так не делайте пожалуйста
fn check_debug1_stages(
    abilities_info: Res<AbilitiesInfo>,
    mut ult_1s_ev_writer: EventWriter<Ult1sDebug1Event>,
    mut ult_2s_ev_writer: EventWriter<Ult2sDebug1Event>,
    mut ult_3s_ev_writer: EventWriter<Ult3sDebug1Event>,
    mut stage_entered_ev: EventReader<StageEntered>,
) {
    for ev in stage_entered_ev.read() {
        if ev.ability != AbilitiesList::Ultimate(UltimatesList::Debug1) {
            return
        }

        println!("start ev(current ability counter = {})", abilities_info.get_stage_counter(ev.ability).get_current_stage());
        match abilities_info.get_stage_counter(ev.ability).get_current_stage() {
            0 => { ult_1s_ev_writer.write(Ult1sDebug1Event); },
            1 => { ult_2s_ev_writer.write(Ult2sDebug1Event); },
            2 => { ult_3s_ev_writer.write(Ult3sDebug1Event(ev.player)); },
            _ => continue
        };
    }
}

fn ul_db1_1s(
    abilities_info: Res<AbilitiesInfo>,
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut Velocity, &Transform), With<Ball>>,
) {
    let (entity, mut velocity, transform) = q_ball.single_mut().unwrap();
    velocity.linvel = Vec2::ZERO;
    velocity.angvel = 25.0;

    let mut rng = rand::rng();
    commands.entity(entity).insert(GameEntityAnimation::new(
        Some(SEVec3::new(transform.translation, vec3(200.0 * rng.random_range(-1.0..=1.0), 100.0 * rng.random_range(-1.0..=1.0), 0.0))),
        None,
        abilities_info.get_current_stage_time(AbilitiesList::Ultimate(UltimatesList::Debug1))
    ));
}

fn ul_db1_2s(
    abilities_info: Res<AbilitiesInfo>,
    mut commands: Commands,
    mut q_ball: Query<(Entity, &mut Velocity, &Transform), With<Ball>>,
) {
    let (entity, mut velocity, transform) = q_ball.single_mut().unwrap();
    velocity.angvel = -10.0;

    let mut rng = rand::rng();
    commands.entity(entity).insert(GameEntityAnimation::new(
        Some(SEVec3::new(transform.translation, vec3(200.0 * rng.random_range(-1.0..=1.0), 100.0 * rng.random_range(-1.0..=1.0), 0.0))),
        None,
        abilities_info.get_current_stage_time(AbilitiesList::Ultimate(UltimatesList::Debug1))
    ));
}

// End
fn ul_db1_3s(
    mut q_ball: Query<&mut Velocity, With<Ball>>,
    mut end_ev: EventReader<Ult3sDebug1Event>,
    q_player: Query<&mut Player>,
) {
    let mut velocity = q_ball.single_mut().unwrap();
    velocity.angvel = 0.0;
    velocity.linvel = vec2(gen_random_velocity_coef() * MAX_START_SPEED_X, gen_random_velocity_coef() * MAX_START_SPEED_Y);

    let mut ppos = PPos::Left;
    for ev in end_ev.read() {
        ppos = ev.0;
    }

    for mut player in q_player {
        if player.position != ppos {
            player.state = PState::Normal;
        }
    }
}