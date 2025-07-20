pub mod score;
pub mod spawn;
pub mod abilities;
mod restart;
mod movement;

use bevy::prelude::*;
use abilities::{ AbilityQueue, Skills, Ultimates };

const SPEED: f32 = 250.0;
const SKILL_DURATION: f32 = 5.0;
const PLAYER_SIZE: Vec2 = Vec2::new(25.0, crate::WINDOW_SIZE.y / 3.5);

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
            movement::MovementPlugin,
            score::ScorePlugin,
            restart::RestartPlugin,
        ));
    }
}

pub enum PPos {
    Right,
    Left
}

#[derive(Component)]
pub struct Player {
    position: PPos,
    skills_queue: AbilityQueue<Skills>,
    skill_timer: Timer,
    ultimates_queue: AbilityQueue<Ultimates>,
    ultimate_progress: u8,
    is_available_ultimate: bool
}

impl Player {
    pub fn new(pos: PPos) -> Self {
        Self { 
            position: pos, ultimate_progress: 0, is_available_ultimate: false,
            skill_timer: Timer::from_seconds(SKILL_DURATION, TimerMode::Once),
            skills_queue: AbilityQueue::new(vec![
                Skills::Debug1,
                Skills::Debug2
            ]),
            ultimates_queue: AbilityQueue::new(vec![
                Ultimates::Debug1,
                Ultimates::Debug2,
            ])
        }
    }

    // Ultimate
    pub fn ultimate_progress(&self) -> u8 {
        self.ultimate_progress
    }

    pub fn increase_ultimate_progression(&mut self) {
        if self.ultimate_progress + 1 > 4 {
            self.is_available_ultimate = true;
            self.ultimate_progress = 0;
            return 
        }

        self.ultimate_progress += 1;
    }

    pub fn use_ultimate(&mut self) {
        if !self.is_available_ultimate {
            return
        }

        //Todo: Create event
        self.is_available_ultimate = false;
        self.ultimates_queue.next();
    }

    pub fn get_ultimate(&self) -> Ultimates {
        self.ultimates_queue.get()
    }

    // Skill
    pub fn get_skills_reloading_progress(&self) -> f32{
        self.skill_timer.elapsed_secs()
    }

    pub fn use_skill(&mut self) {
        if !self.skill_timer.finished() {
            return
        }

        //Todo: Create event
        self.skill_timer.reset();
        self.skills_queue.next();
    }
}
