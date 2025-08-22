pub mod revert;
pub mod shadow;

use bevy::prelude::*;

pub struct SkillsRealizationPlugin;

impl Plugin for SkillsRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            revert::SkillRevertPlugin,
            shadow::SkillShadowPlugin,
        ));
    }
}