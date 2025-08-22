use bevy::{platform::collections::HashMap, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::{
    ge_utils::animation::{
        GameEntityAnimation,
        SEVec3,
    },
    players::{
        abilities::{
            abilities_db::AbilitiesInfo,
            AbilitiesList,
            SkillsList,
            StageEntered
        },
        PPos, Player, PLAYER_SIZE
    }, GameState
};

pub struct SkillShadowPlugin;

impl Plugin for SkillShadowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Skill1sShadowEvent>()
            .add_event::<Skill3sShadowEvent>()
            .add_systems(Update, check_revert_stages.run_if(in_state(GameState::InGame).and(on_event::<StageEntered>)))
            .add_systems(Update, sk_shadow_1s.run_if(in_state(GameState::InGame).and(on_event::<Skill1sShadowEvent>)))
            .add_systems(Update, sk_shadow_3s.run_if(in_state(GameState::InGame).and(on_event::<Skill3sShadowEvent>)))
            .add_systems(Update, despawn_shadow_by_timer.run_if(in_state(GameState::InGame)))
        ;
    }
}

const SHADOW_SCALE: Vec3 = vec3(1.0, 0.6, 1.0);
const SHADOW_DEATH_DURATION: f32 = 1.0;
const SHADOW_DEATH_DURATION_SPARE: f32 = 0.5;

#[derive(Component)]
struct Shadow {
    pos: PPos,
    timer: Timer,
}

#[derive(Event)]
struct Skill1sShadowEvent(PPos);

#[derive(Event)]
struct Skill3sShadowEvent(PPos);

//* -- Shadow systems -- //

fn despawn_shadow_by_timer(
    time: Res<Time>,
    mut commands: Commands,
    q_shadow: Query<(Entity, &mut Shadow, &mut Transform)>,
    q_player: Query<(&Player, &Transform), Without<Shadow>>
) {
    let mut translations = HashMap::<PPos, f32>::new();
    for (player, transform) in q_player {
        translations.insert(player.position, -transform.translation.y);
    }

    for (entity, mut shadow, mut transform) in q_shadow {
        shadow.timer.tick(time.delta());

        // Цикл внутри цикла, ммм... Цикл говнокода продолжается
        transform.translation.y = translations[&shadow.pos];

        if shadow.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

//* -- Ability Stages -- //

// Кто это читает, знайте, что этот код полное дерьмище, конкретно из-за этой части можно побить автора, то есть меня
// Никогда так не делайте пожалуйста
fn check_revert_stages(
    abilities_info: Res<AbilitiesInfo>,
    mut skill_1s_ev_writer: EventWriter<Skill1sShadowEvent>,
    mut skill_3s_ev_writer: EventWriter<Skill3sShadowEvent>,
    mut stage_entered_ev: EventReader<StageEntered>,
) {
    for ev in stage_entered_ev.read() {
        if ev.ability != AbilitiesList::Skill(SkillsList::Shadow) {
            return
        }

        match abilities_info.get_stage_counter(ev.player, ev.ability).get_current_stage() {
            0 => { skill_1s_ev_writer.write(Skill1sShadowEvent(ev.player)); },
            2 => { skill_3s_ev_writer.write(Skill3sShadowEvent(ev.player)); },
            _ => continue,
        };
    }
}

fn sk_shadow_1s(
    abilities_info: Res<AbilitiesInfo>,
    mut commands: Commands,
    mut ev_1s: EventReader<Skill1sShadowEvent>,
    q_player: Query<(Entity, &Player, &Transform)>
) {
    let mut ppos = PPos::Left;
    for ev in ev_1s.read() {
        ppos = ev.0;
    }

    for (entity, player, transform) in q_player {
        if player.position != ppos {
            continue
        }

        commands.entity(entity).insert(GameEntityAnimation::new(
            None,
            Some(SEVec3::new(transform.scale, SHADOW_SCALE)),
            abilities_info.get_current_stage_time(ppos, AbilitiesList::Skill(SkillsList::Shadow))
        ));

        commands.spawn((
            Shadow { 
                pos: player.get_pos(),
                timer: Timer::from_seconds(abilities_info.get_stage_times(AbilitiesList::Skill(SkillsList::Shadow))[1] + SHADOW_DEATH_DURATION + SHADOW_DEATH_DURATION_SPARE, TimerMode::Once),
            },
            Sprite {
                color: Color::BLACK,
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            Transform::from_translation(transform.translation.with_y(-transform.translation.y)),
            RigidBody::Fixed,
            Restitution::coefficient(1.0),
            Friction::coefficient(0.0),
            Collider::cuboid(PLAYER_SIZE.x / 2.0, PLAYER_SIZE.y / 2.0)
        )).insert(GameEntityAnimation::new(
            None,
            Some(SEVec3::new(transform.scale, SHADOW_SCALE)),
            abilities_info.get_current_stage_time(ppos, AbilitiesList::Skill(SkillsList::Shadow))
        ));
    } 
}

// Shadow don't need 2s

fn sk_shadow_3s(
    abilities_info: Res<AbilitiesInfo>,
    mut commands: Commands,
    mut ev_3s: EventReader<Skill3sShadowEvent>,
    q_player: Query<(Entity, &Player, &Transform)>,
    q_shadow: Query<(Entity, &Shadow, &Transform)>,
) {
    let mut ppos = PPos::Left;
    for ev in ev_3s.read() {
        ppos = ev.0;
    }

    for (entity, player, transform) in q_player {
        if player.position != ppos {
            continue
        }

        commands.entity(entity).insert(GameEntityAnimation::new(
            None, 
            Some(SEVec3::new(transform.scale, vec3(1.0, 1.0, 1.0))), 
            abilities_info.get_current_stage_time(ppos, AbilitiesList::Skill(SkillsList::Shadow))
        ));
    }

    for (entity, shadow, transform) in q_shadow {
        if shadow.pos == ppos {
            commands.entity(entity).insert(GameEntityAnimation::new(
                None, 
                Some(SEVec3::new(transform.scale, vec3(0.0, 0.0, 0.0))),
                SHADOW_DEATH_DURATION
            ));
        }
    }
}