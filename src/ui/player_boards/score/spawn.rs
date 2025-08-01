use bevy::prelude::*;

use super::{ Score, PPos, TEXT_SIZE };
use crate::{ GameState, WINDOW_SIZE };

const LAYER_Z: f32 = 1.0;
const OFFSET: Vec2 = vec2(TEXT_SIZE, TEXT_SIZE + 10.0);

const RIGHT_SCORE_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - OFFSET.x, WINDOW_SIZE.y / 2.0 - OFFSET.y, LAYER_Z);
const LEFT_SCORE_POSITION: Vec3 = vec3(-RIGHT_SCORE_POSITION.x, RIGHT_SCORE_POSITION.y, LAYER_Z);

fn spawn_score(mut commands: Commands) {
    // Left score label (negative x)
    commands.spawn((
        Score(PPos::Left),
        Text2d::new("0"),
        TextFont {
            font_size: TEXT_SIZE,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(LEFT_SCORE_POSITION)
    ));

    // Right score label
    commands.spawn((
        Score(PPos::Right),
        Text2d::new("0"),
        TextFont {
            font_size: TEXT_SIZE,
            ..default()
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Transform::from_translation(RIGHT_SCORE_POSITION)
    ));
}

pub struct ScoreSpawnPlugin;

impl Plugin for ScoreSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnMainEntities), spawn_score);
    }
}