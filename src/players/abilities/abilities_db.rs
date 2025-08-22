use std::collections::HashMap;

use bevy::prelude::*;

use crate::players::PPos;

use super::{
    SkillsList,
    UltimatesList,
    AbilitiesList,
};

pub struct AbilitiesDBPlugin;

impl Plugin for AbilitiesDBPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AbilitiesInfo>();
    }
}

#[derive(Resource)]
pub struct AbilitiesInfo(HashMap<AbilitiesList, AbilityInfo>);

//* Обозначение времени фаз всех способностей
impl Default for AbilitiesInfo {
    fn default() -> Self {
        Self(HashMap::from([
            
            // 1s - minus ball velocity
            (AbilitiesList::Skill(SkillsList::Revert), AbilityInfo::new(0, None)),

            /*  
                1s - spawn player shadow and decrease player, shadow scale
                2s - shadow lifetime
                3s - animate increase player scale to normal and despawn shadow
            */
            (AbilitiesList::Skill(SkillsList::Shadow), AbilityInfo::new(2, Some(vec![1.0, 5.0, 1.0]))),

            /*  
                1&2s - animate ball translation to random coords
                3s - return velocity for ball 
            */
            (AbilitiesList::Ultimate(UltimatesList::Debug1), AbilityInfo::new(2, Some(vec![1.5, 1.5]))),
        ]))
    }
}

impl AbilitiesInfo {
    pub fn get_stage_times(&self, ability: AbilitiesList) -> Vec<f32> {
        self.0[&ability].stage_times.clone()
    }

    pub fn get_stage_counter(&self, ppos: PPos, ability: AbilitiesList) -> StageCounter {
        self.0[&ability].counter[&ppos]
    }

    pub fn add_to_counter(&mut self, ppos: PPos, ability: AbilitiesList) -> bool {
        self.0.get_mut(&ability).expect("Ability not found in db").counter.get_mut(&ppos).expect("counter not found").add()
    }

    pub fn get_current_stage_time(&self, ppos: PPos, ability: AbilitiesList) -> f32 {
        if self.0[&ability].stage_times.is_empty() {
            return 0.0
        }

        self.0[&ability].stage_times[self.0[&ability].counter[&ppos].current as usize]
    } 
}

#[derive(Clone, Copy)]
pub struct StageCounter {
    current: u8,
    next: u8,
    count: u8
}

impl StageCounter {
    fn new(count: u8) -> Self {
        Self { current: 0, next: 0, count: count }
    }

    // Return true if counter is to end stage
    pub fn add(&mut self) -> bool {
        if self.next <= self.count {
            self.current = self.next;
            self.next += 1;
        }

        
        if self.next > self.count {
            self.next = 0;
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
    counter: HashMap<PPos, StageCounter>,
    stage_times: Vec<f32>,
}

impl AbilityInfo {
    fn new(stage_count: u8, stage_times: Option<Vec<f32>>) -> Self {
        Self {
            counter: HashMap::from([
                (PPos::Right, StageCounter::new(stage_count)),
                (PPos::Left, StageCounter::new(stage_count)),
            ]),
            stage_times: if let Some(st) = stage_times {
                st
            } else {
                Vec::new()
            }
        }
    }
}
