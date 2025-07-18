use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{Wall, Left, Right};

use crate::{
    ball::Ball,
    players::score::PlayersScore,
    GameState
};

// При столкновении - увеличивает счёт игроку и переводит игру в состояние обновления очков
fn ball_collision(
    mut score: ResMut<PlayersScore>,
    q_ball: Query<Entity, With<Ball>>,
    q_r_wall: Query<Entity, (With<Wall>, With<Right>)>,
    q_l_wall: Query<Entity, (With<Wall>, With<Left>)>,
    mut collisions_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let ball = q_ball.single().unwrap();
    let right_wall = q_r_wall.single().unwrap();
    let left_wall = q_l_wall.single().unwrap();

    for ev in collisions_events.read() {
        match ev {
            CollisionEvent::Started(a, b, _) => {
                if !(*a == ball || *b == ball) {
                    continue
                }

                if (*a == ball && *b == right_wall) || (*a == right_wall && *b == ball) {
                    score.add_point_to_right();
                    next_state.set(GameState::UpdateScore);
                    //println!("Point to the right, in all = {}", score.right_score());
                }

                if (*a == ball && *b == left_wall) || (*a == left_wall && *b == ball) {
                    score.add_point_to_left();
                    next_state.set(GameState::UpdateScore);
                    //println!("Point to the left, in all = {}", score.left_score());
                }
            }
            CollisionEvent::Stopped(_, _, _) => continue
        }
    }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ball_collision.run_if(in_state(GameState::InGame)));
    }
}