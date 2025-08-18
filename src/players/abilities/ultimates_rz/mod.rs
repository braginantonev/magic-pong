pub mod debug1;
pub mod debug2;

use bevy::prelude::*;

pub struct UltimatesRealizationPlugin;

impl Plugin for UltimatesRealizationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug1::Debug1Plugin,
        ));
    }
}