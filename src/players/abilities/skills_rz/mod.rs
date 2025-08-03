pub mod revert;

use bevy::prelude::*;

use super::{ 
    Ability, AbilityStage,
    First, Second, Third, End,
};

pub struct SkillsRealizationPlugin;

impl Plugin for SkillsRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            revert::SkillRevertPlugin,
        ));
    }
}