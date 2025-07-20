use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "ball.png")]
    pub ball: Handle<Image>,

    #[asset(path = "player board.png")]
    pub player_board: Handle<Image>,

    #[asset(path = "border.png")]
    pub border: Handle<Image>
}