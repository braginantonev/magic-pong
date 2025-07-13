mod spawn;
mod collision;

use bevy::prelude::*;

use crate::WINDOW_SIZE;

pub const X_OFFSET: f32 = 100.0;
const HORIZONTAL_WALL_SIZE: Vec2 = vec2(WINDOW_SIZE.x - X_OFFSET * 2.0, 5.0);
const VERTICAL_WALL_SIZE: Vec2 = vec2(5.0, WINDOW_SIZE.y);

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct Left;

#[derive(Component)]
struct Right;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::SpawnPlugin,
            collision::CollisionPlugin,
        ));
    }
}