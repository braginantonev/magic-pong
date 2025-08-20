mod ultimate;
mod ultimates_rz;
mod skill;
mod skills_rz;
mod abilities_db;

use bevy::prelude::*;

use crate::GameState;

use super::PPos;

//* -- Ability Plugin -- */

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StageEntered>()
            .add_systems(Update, tick_stage.run_if(in_state(GameState::InGame)))
            .add_plugins((
                abilities_db::AbilitiesDBPlugin,
                ultimate::UltimatePlugin,
                ultimates_rz::UltimatesRealizationPlugin,
                skill::SkillPlugin,
                skills_rz::SkillsRealizationPlugin,
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

#[derive(Event)]
pub struct StageEntered {
    ability: AbilitiesList,
    player: PPos,
}

#[derive(Component)]
struct Stager;

#[derive(Component)]
struct StageTimer{
    timer: Timer,
    player: PPos,
    ability: AbilitiesList,
}

impl StageTimer {
    pub fn new(player_pos: PPos, ability: AbilitiesList) -> Self {
        StageTimer { timer: Timer::from_seconds(0.0, TimerMode::Once), player: player_pos, ability: ability }
    }
}

fn tick_stage(
    mut ability_info: ResMut<abilities_db::AbilitiesInfo>,
    time: Res<Time>,
    mut commands: Commands,
    mut stage_ev: EventWriter<StageEntered>,
    q_stagers: Query<(Entity, &mut StageTimer), With<Stager>>,
) {
    for (entity, mut stage_timer) in q_stagers {
        stage_timer.timer.tick(time.delta());

        if stage_timer.timer.finished() {
            println!("write event");
            stage_ev.write(StageEntered { ability: stage_timer.ability, player: stage_timer.player });

            if !ability_info.add_to_counter(stage_timer.ability) { // Not in the end stage
                println!("update timer");
                stage_timer.timer = Timer::from_seconds(ability_info.get_current_stage_time(stage_timer.ability), TimerMode::Once);
            } else {
                println!("delete stager with timer");
                commands.entity(entity).despawn();
            }
        }
    }
}

//* -- Abilities functional -- */

pub trait Ability {
    fn to_str(&self) -> String;
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub enum SkillsList {
    Revert,
}

impl Ability for SkillsList {
    fn to_str(&self) -> String {
        match self {
            Self::Revert => "Revert".to_string(),
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
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

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum AbilitiesList {
    Skill(SkillsList),
    Ultimate(UltimatesList)
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