use std::collections::HashMap;

use bevy::prelude::*;

use super::{
    SkillsList,
    UltimatesList,
    AbilitiesList,
};

pub struct AbilitiesInfoPlugin;

impl Plugin for AbilitiesInfoPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilitiesInfo>();
    }
}

#[derive(Resource)]
pub struct AbilitiesInfo(HashMap<AbilitiesList, AbilityInfo>);

impl Default for AbilitiesInfo {
    fn default() -> Self {
        Self(HashMap::from([
            (AbilitiesList::Skill(SkillsList::Revert), AbilityInfo::new(0, None)),
            (AbilitiesList::Ultimate(UltimatesList::Debug1), AbilityInfo::new(2, Some(vec![5.0, 5.0])))
        ]))
    }
}

impl AbilitiesInfo {
    pub fn get_stage_times(&self, ability: AbilitiesList) -> Vec<f32> {
        self.0[&ability].stage_times.clone()
    }

    pub fn get_stage_counter(&self, ability: AbilitiesList) -> StageCounter {
        self.0[&ability].counter
    }
}

#[derive(Clone, Copy)]
struct StageCounter {
    current: u8,
    count: u8
}

impl StageCounter {
    fn new(count: u8) -> Self {
        Self { current: 0, count: count }
    }

    // Return true if counter is to end stage
    pub fn add(&mut self) -> bool {
        if self.current < self.count {
            self.current += 1;
        }

        if self.current == self.count {
            self.current = 0;
            return true
        }

        false
    }

    pub fn get_stages(&self) -> u8 {
        self.count
    } 

    pub fn get_current_stage(&self) -> u8 {
        self.current
    }
}

struct AbilityInfo {
    counter: StageCounter,
    stage_times: Vec<f32>,
}

impl AbilityInfo {
    fn new(stage_count: u8, stage_times: Option<Vec<f32>>) -> Self {
        Self {
            counter: StageCounter::new(stage_count),
            stage_times: if let Some(st) = stage_times {
                st
            } else {
                Vec::new()
            }
        }
    }
}
