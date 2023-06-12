use bevy::prelude::*;

#[derive(Component, Default)]
pub struct ParallaxBackground {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Collider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Pickup {
    pub collider: Collider,
}

#[derive(Component)]
pub struct FuelPickup;

#[derive(Component)]
pub struct HealthPickup;
