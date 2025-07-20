mod player_boards;
mod retart_timer;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            player_boards::PlayerBoardsPlugin,
        ));
    }
}