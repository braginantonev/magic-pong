use bevy::{math::VectorSpace, prelude::*};

use crate::{
    GameState,
    players::{ 
        abilities::{
            UseAbilityEvent,
            Ultimates,
            Skills,
        },
        score::IncreaseScoreEvent,
        Player,
        ULTIMATE_STEPS
    }
};

use super::{
    Placeholder, PPos, PlaceholderType,
    ULTIMATE_PLACEHOLDER_SIZE, ULTIMATE_PLACEHOLDER_POSITION, ULTIMATE_PLACEHOLDER_MAX_Y,
    SKILL_PLACEHOLDER_SIZE, SKILL_PLACEHOLDER_POSITION, SKILL_PLACEHOLDER_MAX_Y
};

const ULTIMATE_DURATION: f32 = 1.0;
const ULTIMATE_STEP_SIZE: f32 = ULTIMATE_PLACEHOLDER_MAX_Y / ULTIMATE_PLACEHOLDER_SIZE.y / ULTIMATE_STEPS as f32;

#[derive(Component)]
struct PlaceholderAnimation {
    target_scale: Vec3,
    start_scale: Vec3,
    target_position: Vec3,
    start_position: Vec3,
    timer: Timer
}

fn clear_ultimate_placeholder(
    mut commands: Commands,
    q_placeholders: Query<(Entity, &Transform, &Placeholder), Without<PlaceholderAnimation>>,
    mut use_ultimate_event: EventReader<UseAbilityEvent<Ultimates>>
) {
    if use_ultimate_event.is_empty() {
        return
    }

    let mut placeholder_pos: PPos = PPos::Left;

    for ev in use_ultimate_event.read() {
        placeholder_pos = ev.get_pos()
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
            timer: Timer::from_seconds(ULTIMATE_DURATION, TimerMode::Once)
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
            continue
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
                timer: Timer::from_seconds(ULTIMATE_DURATION, TimerMode::Once)
            });
        }
    }
}

fn animate_placeholders(
    mut commands: Commands,
    time: Res<Time>,
    q_placeholders: Query<(Entity, &mut Transform, &mut PlaceholderAnimation), With<Placeholder>>,
) {
    for (entity, mut transform, mut anim) in q_placeholders {
        anim.timer.tick(time.delta());

        transform.scale = anim.start_scale.lerp(anim.target_scale, anim.timer.fraction());
        transform.translation = anim.start_position.lerp(anim.target_position, anim.timer.fraction());

        if anim.timer.finished() {
            commands.entity(entity).remove::<PlaceholderAnimation>();
        }
    }
}

pub struct PlaceholderUpdatePlugin;

impl Plugin for PlaceholderUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Restart), increase_ultimate_placeholder)
            .add_systems(Update, animate_placeholders.run_if(in_state(GameState::Restart)).after(increase_ultimate_placeholder))
            .add_systems(Update, clear_ultimate_placeholder.run_if(in_state(GameState::InGame)));
    }
}