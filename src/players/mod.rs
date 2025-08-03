pub mod score;
pub mod spawn;
pub mod abilities;
mod restart;
mod movement;

use bevy::prelude::*;
use abilities::{ AbilityQueue, SkillsList, UltimatesList };

pub const ULTIMATE_STEPS: u8 = 5;
pub const SKILL_REFRESH_DURATION: f32 = 10.0;

const SPEED: f32 = 250.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, crate::WINDOW_SIZE.y / 3.5);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                spawn::SpawnPlugin,
                movement::MovementPlugin,
                score::ScorePlugin,
                restart::RestartPlugin,
                abilities::AbilityPlugin,
            ));
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum PPos {
    Right,
    Left
}

impl PPos {
    pub fn negative(&self) -> Self {
        match self {
            PPos::Right => PPos::Left,
            PPos::Left => PPos::Right
        }
    }
}

#[derive(Component)]
pub struct Player {
    position: PPos,

    skills_queue: AbilityQueue<SkillsList>,
    skill_timer: Timer,

    ultimates_queue: AbilityQueue<UltimatesList>,
    ultimate_progress: u8,
}

impl Player {
    pub fn new(pos: PPos) -> Self {
        Self { 
            position: pos, ultimate_progress: 0,
            skill_timer: Timer::from_seconds(SKILL_REFRESH_DURATION, TimerMode::Once),
            skills_queue: AbilityQueue::new(vec![
                SkillsList::Revert,
            ]),
            ultimates_queue: AbilityQueue::new(vec![
                UltimatesList::Debug1,
                UltimatesList::Debug2,
            ])
        }
    }

    pub fn get_pos(&self) -> PPos {
        self.position
    }

    // Ultimate
    pub fn ultimate_progress(&self) -> u8 {
        self.ultimate_progress
    }

    pub fn ultimate_is_available(&self) -> bool {
        self.ultimate_progress == ULTIMATE_STEPS
    }

    pub fn increase_ultimate_progression(&mut self) {
        if self.ultimate_is_available() {
            return
        }
        self.ultimate_progress += 1;
    }

    // Return used ultimate
    pub fn use_ultimate(&mut self) -> Option<UltimatesList> {
        if !self.ultimate_is_available() {
            return None
        }
        
        let used_ult = self.get_ultimate();

        self.ultimate_progress = 0;
        self.ultimates_queue.next();

        Some(used_ult)
    }

    pub fn get_ultimate(&self) -> UltimatesList {
        self.ultimates_queue.get()
    }

    // Skill
    pub fn skill_is_available(&self) -> bool {
        self.skill_timer.finished()
    }

    pub fn get_skill_timer_fraction(&self) -> f32 {
        self.skill_timer.fraction()
    }

    pub fn use_skill(&mut self) -> Option<SkillsList> {
        if !self.skill_is_available() {
            println!("skill not available");
            return None
        }

        let skill = self.skills_queue.get();

        self.skill_timer.reset();
        self.skills_queue.next();

        Some(skill)
    }
}
