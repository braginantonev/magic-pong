use bevy::prelude::*;

use crate::{ GameState, assets::GameAssets };

fn spawn_border(
    assets: Res<GameAssets>,
    mut commands: Commands
) {
    commands.spawn((
        Sprite {
            image: assets.border.clone(),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, -1.0)
    ));
}

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnMainEntities), spawn_border);
    }
}