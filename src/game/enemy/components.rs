use bevy::prelude::*;

use super::{ENEMY_MAX_HEALTH, ENEMY_SPEED};

#[derive(Component)]
pub struct Enemy {
    pub current_hp: f32,
    pub speed: f32,
    pub state: EnemyState,

    pub direction: Vec3,
    pub destination: Vec3,
    pub destination_reached: bool,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            current_hp: ENEMY_MAX_HEALTH,
            speed: ENEMY_SPEED,
            state: EnemyState::SPAWNED,
            direction: Vec3::ZERO,
            destination: Vec3::ZERO,
            destination_reached: false,
        }
    }
}

#[derive(Default, PartialEq)]
pub enum EnemyState {
    #[default]
    SPAWNED,
    ENGAGING,
}

#[derive(Component)]
pub struct FollowAI;
