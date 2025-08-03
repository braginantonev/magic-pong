use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ 
    Ability, AbilityStage,
    First
};

use crate::GameState;

pub struct Debug1Plugin;

impl Plugin for Debug1Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<AbilityStage<Debug1, First>>()
            .add_systems(Update, db1_first_stage.run_if(in_state(GameState::InGame)).run_if(on_event::<AbilityStage<Debug1, First>>));
    }
}

pub struct Debug1;

impl Ability for Debug1 {
    fn to_str(&self) -> String {
        "Debug 1".to_string()
    }
}

fn db1_first_stage(
    asr_event: EventReader<AbilityStage<Debug1, First>>
) {
    
}