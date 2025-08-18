use std::collections::HashMap;

use bevy::prelude::*;

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
            (AbilitiesList::Skill(SkillsList::Revert), AbilityInfo::new(0, None)),
            (AbilitiesList::Ultimate(UltimatesList::Debug1), AbilityInfo::new(2, Some(vec![1.5, 1.5])))
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

    pub fn add_to_counter(&mut self, ability: AbilitiesList) -> bool {
        self.0.get_mut(&ability).expect("Ability not found in db").counter.add()
    }

    pub fn get_current_stage_time(&self, ability: AbilitiesList) -> f32 {
        if self.0[&ability].stage_times.is_empty() {
            println!("hello");
            return 0.0
        }

        self.0[&ability].stage_times[self.0[&ability].counter.current as usize]
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
            println!("add to counter(current: {}, next: {})", self.current, self.next);
            self.current = self.next;
            self.next += 1;
            println!("successfully added(current: {}, next: {})", self.current, self.next);
        }

        
        if self.next > self.count {
            println!("break counter(current: {})", self.current);
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
