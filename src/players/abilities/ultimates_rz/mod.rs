pub mod debug1;
pub mod debug2;

use bevy::prelude::*;

use super::{ 
    Ability, AbilityStage,
    First, Second, Third, End,
};

pub struct UltimatesRealizationPlugin;

impl Plugin for UltimatesRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug1::Debug1Plugin,
        ));
    }
}