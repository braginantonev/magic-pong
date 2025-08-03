mod ultimate;
mod ultimates_list;
mod skill;

use bevy::prelude::*;

use super::PPos;

#[derive(Event)]
pub struct UseAbilityEvent<T: Ability> {
    pos: PPos,
    ability: T
}

impl<T: Ability + Copy> UseAbilityEvent<T> {
    pub fn get_pos(&self) -> PPos {
        self.pos
    }

    pub fn get_ability(&self) -> T {
        self.ability
    }
}

//* -- Ability Plugin -- */

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ultimate::UltimatePlugin,
                skill::SkillPlugin
            )); 
    }
}

//* -- Abilities functional -- */
pub trait Ability {
    fn to_str(&self) -> String;
}

#[derive(Clone, Copy)]
pub enum SkillsList {
    Debug1,
    Debug2,
}

impl Ability for SkillsList {
    fn to_str(&self) -> String {
        match self {
            SkillsList::Debug1 => "Debug 1".to_string(),
            SkillsList::Debug2 => "Debug 2".to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UltimatesList {
    Debug1,
    Debug2,
}

impl Ability for UltimatesList {
    fn to_str(&self) -> String {
        match self {
            UltimatesList::Debug1 => "Debug 1".to_string(),
            UltimatesList::Debug2 => "Debug 2".to_string(),
        }
    }
}

pub struct AbilityQueue<T: Ability> {
    queue: Vec<T>,
    current_ability: usize
}

impl<T: Ability + Copy> AbilityQueue<T> {
    pub fn new(abilities: Vec<T>) -> Self {
        AbilityQueue { queue: abilities, current_ability: 0 }
    }

    pub fn get(&self) -> T {
        self.queue[self.current_ability]
    }

    pub fn next(&mut self) {
        if self.current_ability + 1 >= self.queue.len() {
            self.current_ability = 0;
            return
        }
        self.current_ability += 1;
    }
}