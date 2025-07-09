mod spawn;

use bevy::prelude::*;

#[derive(Component)]
struct Wall;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn::SpawnPlugin);
    }
}