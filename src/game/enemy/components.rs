use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Enemy {
    pub direction: Vec3,
    pub destination: Vec3,
}
