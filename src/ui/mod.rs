mod player_boards;
mod retart_timer;
mod border;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            border::BorderPlugin,
            player_boards::PlayerBoardsPlugin,
        ));
    }
}