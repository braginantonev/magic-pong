use bevy::prelude::*;

use super::{ Score, Type, TEXT_SIZE };
use crate::{ GameState, WINDOW_SIZE };

const OFFSET: Vec2 = vec2(TEXT_SIZE, TEXT_SIZE + 10.0);
const LEFT_SCORE_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - OFFSET.x, WINDOW_SIZE.y / 2.0 - OFFSET.y, 0.0);
const RIGHT_SCORE_POSITION: Vec3 = vec3(-LEFT_SCORE_POSITION.x, LEFT_SCORE_POSITION.y, 0.0);

fn spawn_score(mut commands: Commands) {
    // Left score label (negative x)
    commands.spawn((
        Score::new(Type::Left),
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
        Score::new(Type::Right),
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