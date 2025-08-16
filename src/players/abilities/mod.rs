mod ultimate;
mod ultimates_rz;
mod skill;
mod skills_rz;

use bevy::prelude::*;

use crate::GameState;

use super::PPos;

//* -- Ability Plugin -- */

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::SpawnMainEntities), spawn_stager)
            .add_plugins((
                ultimate::UltimatePlugin,
                ultimates_rz::UltimatesRealizationPlugin,
                skill::SkillPlugin,
                skills_rz::SkillsRealizationPlugin
            )); 
    }
}

//* -- Events -- */

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

//* -- Ability Stages -- */

#[derive(Clone, Copy)]
enum Stages {
    First,
    Second,
    Third,
    End
}

trait Stage {}

struct First;
impl Stage for First {}

struct Second;
impl Stage for Second {}

struct Third;
impl Stage for Third {}

struct End;
impl Stage for End {}

struct StageInfo {
    stage: Stages,
    duration: Option<f32>
}

impl StageInfo {
    fn end() -> Self {
        StageInfo { stage: Stages::End, duration: None }
    }
}

struct StageQuery(Vec<StageInfo>);

impl StageQuery {
    fn get_and_next(&mut self) -> StageInfo {
        if self.0.is_empty() {
            return StageInfo::end()
        } 

        self.0.remove(0)
    }
}

#[derive(Component)]
struct AbilityStager;

#[derive(Component)]
struct StageTimer {
    timer: Timer,
    stage_query: StageQuery 
}

#[derive(Component)]
struct AbilityStage<A: Ability, S: Stage> {
    player: PPos,

    _ability: A,
    _stage: S
}

struct ObjectAnimation {
    start_position: Vec3,
    start_scale: Vec2,

    target_position: Vec3,
    target_scale: Vec2,

    timer: Timer
}

#[derive(Component)]
struct AbilityStageAnimator(Vec<ObjectAnimation>);

//Todo: Доделать аниматор
fn tick_stage(
    mut commands: Commands,
    time: Res<Time>,
    //q_stage_timer: Query<&mut AbilityStageTimer>,
) {
    for mut stage_timer in q_stage_timer {
        stage_timer.timer.tick(time.delta());

        if stage_timer.timer.finished() {
            
        }
    }
}

fn spawn_stager(
    mut commands: Commands
) {
    commands.spawn((
        AbilityStager,
        AbilityStageAnimator(Vec::new())
    ));
}


//* -- Abilities functional -- */

pub trait Ability {
    fn to_str(&self) -> String;
}

#[derive(Clone, Copy)]
pub enum SkillsList {
    Revert,
}

impl Ability for SkillsList {
    fn to_str(&self) -> String {
        match self {
            SkillsList::Revert => "Revert".to_string(),
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