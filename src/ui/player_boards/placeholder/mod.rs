mod spawn;
mod update;

use bevy::prelude::*;

use crate::{
    WINDOW_SIZE,
    ui::player_boards::PPos
};

use super::BOARD_SIZE;

const LAYER_Z: f32 = -1.0;

const ULTIMATE_PLACEHOLDER_MAX_Y: f32 = 95.0;
const ULTIMATE_PLACEHOLDER_SIZE: Vec2 = vec2(92.0, 5.0);
const ULTIMATE_PLACEHOLDER_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0, -65.0, LAYER_Z);

const SKILL_PLACEHOLDER_MAX_Y: f32 = 80.0;
const SKILL_PLACEHOLDER_SIZE: Vec2 = vec2(80.0, 0.1);
const SKILL_PLACEHOLDER_POSITION: Vec3 = vec3(WINDOW_SIZE.x / 2.0 - BOARD_SIZE.x / 2.0, -165.0, LAYER_Z);

#[derive(PartialEq)]
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

pub struct PlaceholderPlugin;

impl Plugin for PlaceholderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::PlaceholderSpawnPlugin,
            update::PlaceholderUpdatePlugin
        ));
    }
}