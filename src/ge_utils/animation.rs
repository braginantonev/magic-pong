use bevy::prelude::*;

use crate::GameState;

pub struct GEAnimationPlugin;

impl Plugin for GEAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (tick_animations, animate_entities).chain().run_if(in_state(GameState::InGame)));
    }
}

fn tick_animations(
    time: Res<Time>,
    mut commands: Commands,
    q_ge_animations: Query<(Entity, &mut GameEntityAnimation)>
) {
    for (entity, mut anim) in q_ge_animations {
        anim.timer.tick(time.delta());

        if anim.timer.finished() {
            commands.entity(entity).remove::<GameEntityAnimation>();
        }
    }
}

fn animate_entities(
    q_ball: Query<(&mut Transform, &GameEntityAnimation)>,
) {
    for (mut transform, anim) in q_ball {
        transform.translation = anim.lerp_position();
        transform.scale = anim.lerp_scale();
    }
}

/// Start End Vec3 struct. 
/// Use to add start and end for animation
#[derive(Clone, Copy)]
pub struct SEVec3 {
    start: Vec3,
    target: Vec3,
} 

impl SEVec3 {
    pub fn new(s: Vec3, t: Vec3) -> Self {
        Self { start: s, target: t }
    }

    pub fn new_without_anim(base: Vec3) -> Self {
        Self { start: base, target: base }
    }
}

#[derive(Component)]
pub struct GameEntityAnimation {
    pos: Option<SEVec3>,
    scale: Option<SEVec3>,
    pub timer: Timer,
}

impl GameEntityAnimation {
    pub fn lerp_position(&self) -> Vec3 {
        let pos = self.pos.unwrap();
        pos.start.lerp(pos.target, self.timer.fraction())
    }

    pub fn lerp_scale(&self) -> Vec3 {
        let scale = self.scale.unwrap();
        scale.start.lerp(scale.target, self.timer.fraction())
    }

    pub fn new(
        position: SEVec3,
        scale: SEVec3,
        duration: f32,
    ) -> Self {
        Self { pos: Some(position), scale: Some(scale), timer: Timer::from_seconds(duration, TimerMode::Once) }
    }
}