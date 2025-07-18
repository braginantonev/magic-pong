use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{ ActiveWall, Position };

use crate::{
    ball::Ball,
    players::score::PlayersScore,
    GameState
};

// При столкновении - увеличивает счёт игроку и переводит игру в состояние обновления очков
fn ball_collision(
    mut score: ResMut<PlayersScore>,
    q_ball: Query<Entity, With<Ball>>,
    q_wall: Query<(Entity, &ActiveWall)>,
    mut collisions_events: EventReader<CollisionEvent>,
    mut next_state: ResMut<NextState<GameState>>
) {
    let ball = q_ball.single().unwrap();

    for ev in collisions_events.read() {
        match ev {
            CollisionEvent::Started(a, b, _) => {
                if !(*a == ball || *b == ball) {
                    continue
                }

                for (entity, wall) in q_wall {
                    if (*a == ball && *b == entity) || (*a == entity && *b == ball) {
                        match wall.0 {
                            Position::Right => score.add_point_to_right(),
                            Position::Left => score.add_point_to_left(),
                        }
                    
                        next_state.set(GameState::UpdateScore);
                        break;
                    }
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