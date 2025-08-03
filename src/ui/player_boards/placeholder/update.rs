use bevy::prelude::*;

use crate::{
    players::{ 
        abilities::{
            SkillsList, UltimatesList, UseAbilityEvent
        },
        score::IncreaseScoreEvent,
        Player,
        ULTIMATE_STEPS
    }, GameState
};

use super::{
    Placeholder, PPos, PlaceholderType, PlaceholderAnimation,
    ULTIMATE_PLACEHOLDER_SIZE, ULTIMATE_PLACEHOLDER_POSITION, ULTIMATE_PLACEHOLDER_MAX_Y,
    SKILL_PLACEHOLDER_POSITION
};

const ULTIMATE_DURATION: f32 = 1.0;
const ULTIMATE_STEP_SIZE: f32 = ULTIMATE_PLACEHOLDER_MAX_Y / ULTIMATE_PLACEHOLDER_SIZE.y / ULTIMATE_STEPS as f32;

fn clear_skill_placeholder(
    mut commands: Commands,
    q_placeholders: Query<(Entity, &Transform, &Placeholder), Without<PlaceholderAnimation>>,
    mut use_skill_event: EventReader<UseAbilityEvent<SkillsList>>
) {
    if use_skill_event.is_empty() {
        return
    }

    let mut placeholder_pos: PPos = PPos::Left;

    for ev in use_skill_event.read() {
        placeholder_pos = ev.get_pos();
    }

    for (entity, transform, placeholder) in q_placeholders {
        if placeholder.typ != PlaceholderType::Skill {
            continue;
        }

        if placeholder.position != placeholder_pos {
            continue;
        }

        commands.entity(entity).insert(PlaceholderAnimation {
            start_position: transform.translation,
            start_scale: transform.scale,
            target_scale: vec3(1.0, 1.0, 1.0),
            target_position: match placeholder_pos {
                PPos::Right => SKILL_PLACEHOLDER_POSITION,
                PPos::Left => SKILL_PLACEHOLDER_POSITION.with_x(-SKILL_PLACEHOLDER_POSITION.x)
            },
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            reload_needed: true
        });
    }
}

fn clear_ultimate_placeholder(
    mut commands: Commands,
    q_placeholders: Query<(Entity, &Transform, &Placeholder), Without<PlaceholderAnimation>>,
    mut use_ultimate_event: EventReader<UseAbilityEvent<UltimatesList>>
) {
    if use_ultimate_event.is_empty() {
        return
    }

    let mut placeholder_pos: PPos = PPos::Left;

    for ev in use_ultimate_event.read() {
        placeholder_pos = ev.get_pos();
    }

    for (entity, transform, placeholder) in q_placeholders {
        if placeholder.typ != PlaceholderType::Ultimate {
            continue;
        }

        if placeholder.position != placeholder_pos {
            continue;
        }

        commands.entity(entity).insert(PlaceholderAnimation {
            start_position: transform.translation,
            start_scale: transform.scale,
            target_scale: transform.scale.with_y(ULTIMATE_PLACEHOLDER_SIZE.y),
            target_position: transform.translation.with_y(ULTIMATE_PLACEHOLDER_POSITION.y),
            timer: Timer::from_seconds(ULTIMATE_DURATION, TimerMode::Once),
            reload_needed: false
        });
    }
}

fn increase_ultimate_placeholder(
    mut commands: Commands,
    q_players: Query<&Player>,
    q_placeholders: Query<(Entity, &Transform, &Placeholder), Without<PlaceholderAnimation>>,
    mut increase_score_event: EventReader<IncreaseScoreEvent>
) {
    if increase_score_event.is_empty() {
        return
    }

    let mut increase_placeholder_pos: PPos = PPos::Left;

    for ev in increase_score_event.read() {
        increase_placeholder_pos = ev.0
    }

    for (entity, transform, placeholder) in q_placeholders {
        if placeholder.typ != PlaceholderType::Ultimate {
            continue
        }

        if placeholder.position != increase_placeholder_pos {
            continue;
        }

        for player in q_players {
            if player.get_pos() != placeholder.position {
                continue
            }

            if player.ultimate_is_available() {
                return
            }

            commands.entity(entity).insert(PlaceholderAnimation {
                start_position: transform.translation,
                start_scale: transform.scale,
                target_scale: transform.scale.with_y(transform.scale.y + ULTIMATE_STEP_SIZE),
                target_position: transform.translation.with_y(transform.translation.y + ULTIMATE_PLACEHOLDER_MAX_Y / ULTIMATE_STEPS as f32 / 2.0),
                timer: Timer::from_seconds(ULTIMATE_DURATION, TimerMode::Once),
                reload_needed: false
            });
        }
    }
}


// Это разделение на анимацию ультимейта и скила такое дерьмо
// Хочу сломать себе пальцы и заглянуть к себе в мозг
fn animate_ult_placeholders(
    mut commands: Commands,
    time: Res<Time>,
    q_placeholders: Query<(Entity, &mut Transform, &mut PlaceholderAnimation, &Placeholder)>,
) {
    for (entity, mut transform, mut anim, placeholder) in q_placeholders {
        if placeholder.typ != PlaceholderType::Ultimate {
            continue;
        }

        anim.timer.tick(time.delta());

        transform.scale = anim.start_scale.lerp(anim.target_scale, anim.timer.fraction());
        transform.translation = anim.start_position.lerp(anim.target_position, anim.timer.fraction());

        if anim.timer.finished() {
            commands.entity(entity).remove::<PlaceholderAnimation>();
        }
    }
}

fn animate_skill_placeholders(
    mut commands: Commands,
    time: Res<Time>,
    q_placeholders: Query<(Entity, &mut Transform, &mut PlaceholderAnimation, &Placeholder)>,
) {
    for (entity, mut transform, mut anim, placeholder) in q_placeholders {
        if placeholder.typ != PlaceholderType::Skill {
            continue;
        }

        anim.timer.tick(time.delta());

        transform.scale = anim.start_scale.lerp(anim.target_scale, anim.timer.fraction());
        transform.translation = anim.start_position.lerp(anim.target_position, anim.timer.fraction());

        if anim.timer.finished() {
            
            //* This is a crutch for skill placeholder
            //Todo: need optimization
            if anim.reload_needed {
                *anim = PlaceholderAnimation::skill_default(placeholder.position);
                continue;
            }

            commands.entity(entity).remove::<PlaceholderAnimation>();
        }
    }
}

pub struct PlaceholderUpdatePlugin;

impl Plugin for PlaceholderUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Restart), increase_ultimate_placeholder)
            .add_systems(Update, animate_ult_placeholders.run_if(in_state(GameState::Restart)).after(increase_ultimate_placeholder))
            .add_systems(Update, (clear_ultimate_placeholder, clear_skill_placeholder, animate_skill_placeholders).run_if(in_state(GameState::InGame)));
    }
}