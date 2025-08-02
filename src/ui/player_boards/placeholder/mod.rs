mod spawn;
mod update;

use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    players::SKILL_REFRESH_DURATION,
    ui::player_boards::PPos
};

use super::BOARD_SIZE;

const LAYER_Z: f32 = -1.0;

const ULTIMATE_PLACEHOLDER_MAX_Y: f32 = 95.0;
const ULTIMATE_PLACEHOLDER_SIZE: Vec2 = vec2(92.0, 5.0);
const ULTIMATE_PLACEHOLDER_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0, -65.0, LAYER_Z);

const SKILL_PLACEHOLDER_MAX_Y: f32 = 90.0;
const SKILL_PLACEHOLDER_SIZE: Vec2 = vec2(80.0, 0.1);
const SKILL_PLACEHOLDER_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0, -210.0, LAYER_Z);

#[derive(PartialEq, Debug)]
enum PlaceholderType {
    Skill,
    Ultimate,
    //Available
}

#[derive(Component)]
struct Placeholder {
    position: PPos,
    typ: PlaceholderType,
}

impl Placeholder {
    fn new(pos: PPos, p_type: PlaceholderType) -> Placeholder {
        Placeholder { position: pos, typ: p_type }
    }
}

#[derive(Component)]
struct PlaceholderAnimation {
    target_scale: Vec3,
    start_scale: Vec3,
    target_position: Vec3,
    start_position: Vec3,
    timer: Timer,

    //* This is a crutch for skill placeholder
    //Todo: need optimization
    reload_needed: bool
}

impl PlaceholderAnimation {
    // Return default animation for right placeholder
    pub fn skill_default(pos: PPos) -> Self {
        let k: f32 = match pos {
            PPos::Left => -1.0,
            PPos::Right => 1.0
        };

        PlaceholderAnimation { 
            start_position: SKILL_PLACEHOLDER_POSITION.with_x(SKILL_PLACEHOLDER_POSITION.x * k),
            start_scale: vec3(1.0, 1.0, 1.0),
            target_position: vec3(SKILL_PLACEHOLDER_POSITION.x * k, SKILL_PLACEHOLDER_POSITION.y + SKILL_PLACEHOLDER_MAX_Y / 2.0, SKILL_PLACEHOLDER_POSITION.z),
            target_scale: vec3(1.0, SKILL_PLACEHOLDER_MAX_Y / SKILL_PLACEHOLDER_SIZE.y, 1.0),
            timer: Timer::from_seconds(SKILL_REFRESH_DURATION - 0.5, TimerMode::Once),
            reload_needed: false
        }
    }
}

pub struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::PlaceholderSpawnPlugin,
            update::PlaceholderUpdatePlugin
        ));
    }
}