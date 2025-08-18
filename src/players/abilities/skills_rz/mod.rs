pub mod revert;

use bevy::prelude::*;

pub struct SkillsRealizationPlugin;

impl Plugin for SkillsRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            revert::SkillRevertPlugin,
        ));
    }
}