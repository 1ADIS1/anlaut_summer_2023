use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub current_hp: f32,
    pub direction: Vec3,
    pub destination: Vec3,
}
