pub mod animation;

use bevy::prelude::*;

pub struct GEUtilsPlugin;

impl Plugin for GEUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            animation::GEAnimationPlugin,
        ));
    }
}