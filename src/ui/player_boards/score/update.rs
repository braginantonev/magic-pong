use bevy::prelude::*;

use crate::{ GameState, players::score::{ PlayersScore, IncreaseScoreEvent } };
use super::{ Score, PPos };

const ANIMATION_DURATION: f32 = crate::world::score::UPDATE_SCORE_DURATION / 2.0;

#[derive(Component)]
pub struct ScaleAnimation {
    start_scale: Vec3,
    target_scale: Vec3,
    timer: Timer
}

fn update_score(
    mut commands: Commands,
    players_score: Res<PlayersScore>,
    mut score_event: EventReader<IncreaseScoreEvent>,
    q_score: Query<(Entity, &Score, &mut Text2d, &mut Transform), Without<ScaleAnimation>>
) {
    if score_event.is_empty() {
        return
    }

    let mut pos_update: PPos = PPos::Left;

    for ev in score_event.read() {
        pos_update = ev.0;
    }

    for (entity, score, mut text, mut transform) in q_score {
        if score.0 != pos_update {
            continue
        }

        text.0 = players_score.get(pos_update).to_string();
        transform.scale *= 2.0;

        commands.entity(entity).insert(ScaleAnimation {
            start_scale: transform.scale,
            target_scale:transform.scale / 2.0,
            timer: Timer::from_seconds(ANIMATION_DURATION, TimerMode::Once)
        });
    }
}

fn animate_scale_score(
    mut commands: Commands,
    time: Res<Time>,
    q_score: Query<(Entity, &mut Transform, &mut ScaleAnimation), With<Score>>
) {
    for (entity, mut transform, mut anim) in q_score {
        anim.timer.tick(time.delta());

        transform.scale = anim.start_scale.lerp(anim.target_scale, anim.timer.fraction());

        if anim.timer.finished() {
            commands.entity(entity).remove::<ScaleAnimation>();
        }
    }
}

pub struct ScoreUpdatePlugin;

impl Plugin for ScoreUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Restart), update_score)
            .add_systems(Update, animate_scale_score.run_if(in_state(GameState::Restart)).after(update_score));
    }
}