use bevy::prelude::*;

use crate::{ GameState, players::score::PlayersScore };
use super::{ Score, Type };

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
    q_score: Query<(Entity, &mut Score, &mut Text2d, &mut Transform)>
) {
    for (entity, mut score, mut text, mut transform) in q_score {
        match score.r#type {
            Type::Left => {
                let player_score = players_score.left_score().to_string();
                if text.0 != player_score {
                    score.need_update = true;
                    text.0 = player_score;
                } else { 
                    continue;
                }
            },
            Type::Right => {
                let player_score = players_score.right_score().to_string();
                if text.0 != player_score {
                    score.need_update = true;
                    text.0 = player_score;
                } else { 
                    continue;
                }
            },
        }

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
    q_score: Query<(Entity, &Score, &mut Transform, &mut ScaleAnimation)>
) {
    for (entity, score, mut transform, mut anim) in q_score {
        if !score.need_update {
            continue;
        }

        anim.timer.tick(time.delta());

        transform.scale = anim.start_scale.lerp(anim.target_scale, anim.timer.elapsed_secs());

        if anim.timer.finished() {
            commands.entity(entity).remove::<ScaleAnimation>();
        }
    }
}

pub struct ScoreUpdatePlugin;

impl Plugin for ScoreUpdatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::UpdateScore), update_score)
            .add_systems(Update, animate_scale_score.run_if(in_state(GameState::UpdateScore)));
    }
}