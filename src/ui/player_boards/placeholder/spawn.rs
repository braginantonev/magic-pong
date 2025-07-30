use bevy::prelude::*;

use crate::GameState;
use super::{
    Placeholder, PPos, PlaceholderType, 
    ULTIMATE_PLACEHOLDER_SIZE, ULTIMATE_PLACEHOLDER_POSITION, SKILL_PLACEHOLDER_SIZE, SKILL_PLACEHOLDER_POSITION
};

const ULTIMATE_PLACEHOLDER_COLOR: Color = Color::BLACK;
const SKILL_PLACEHOLDER_COLOR: Color = Color::WHITE;

fn spawn_placeholders(
    mut commands: Commands
) {
    // Left ultimate placeholder
    commands.spawn((
        Placeholder::new(PPos::Left, PlaceholderType::Ultimate),
        Sprite {
            color: ULTIMATE_PLACEHOLDER_COLOR,
            custom_size: Some(ULTIMATE_PLACEHOLDER_SIZE),
            ..default()
        },
        Transform::from_translation(ULTIMATE_PLACEHOLDER_POSITION)
    ));

    // Left skill placeholder
    commands.spawn((
        Placeholder::new(PPos::Left, PlaceholderType::Skill),
        Sprite {
            color: SKILL_PLACEHOLDER_COLOR,
            custom_size: Some(SKILL_PLACEHOLDER_SIZE),
            ..default()
        },
        Transform::from_translation(SKILL_PLACEHOLDER_POSITION)
    ));

    // Right ultimate placeholder
    commands.spawn((
        Placeholder::new(PPos::Right, PlaceholderType::Ultimate),
        Sprite {
            color: ULTIMATE_PLACEHOLDER_COLOR,
            custom_size: Some(ULTIMATE_PLACEHOLDER_SIZE),
            ..default()
        },
        Transform::from_translation(ULTIMATE_PLACEHOLDER_POSITION.with_x(-ULTIMATE_PLACEHOLDER_POSITION.x))
    ));

    // Right skill placeholder
    commands.spawn((
        Placeholder::new(PPos::Right, PlaceholderType::Skill),
        Sprite {
            color: SKILL_PLACEHOLDER_COLOR,
            custom_size: Some(SKILL_PLACEHOLDER_SIZE),
            ..default()
        },
        Transform::from_translation(SKILL_PLACEHOLDER_POSITION.with_x(-SKILL_PLACEHOLDER_POSITION.x))
    ));
}

pub struct PlaceholderSpawnPlugin;

impl Plugin for PlaceholderSpawnPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SpawnMainEntities), spawn_placeholders);
    }
}