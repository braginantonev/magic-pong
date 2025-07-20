//use bevy::prelude::*;

pub trait Ability {
    fn to_str(&self) -> String;
}

#[derive(Clone, Copy)]
pub enum Skills {
    Debug1,
    Debug2,   
}

impl Ability for Skills {
    fn to_str(&self) -> String {
        match self {
            Skills::Debug1 => "Debug 1".to_string(),
            Skills::Debug2 => "Debug 2".to_string()
        }
    }
}

#[derive(Clone, Copy)]
pub enum Ultimates {
    Debug1,
    Debug2
}

impl Ability for Ultimates {
    fn to_str(&self) -> String {
        match self {
            Ultimates::Debug1 => "Debug 1".to_string(),
            Ultimates::Debug2 => "Debug 2".to_string()
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