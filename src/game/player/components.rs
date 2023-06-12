use crate::game::components::Collider;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub current_speed: f32,
    pub collider: Collider,
}
