use super::{EnemyType, FIRE_DURATION};
use crate::game::components::Collider;

use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub max_hp: f32,
    pub current_hp: f32,

    pub default_speed: f32,
    pub current_speed: f32,

    pub enemy_type: EnemyType,
    pub depth_level: f32,

    pub collider: Collider,
    pub state: EnemyState,

    pub direction: Vec3,
    pub destination: Vec3,

    pub destination_reached: bool,

    // Used when the enemy is on fire and it becomes reddish
    pub is_green_decreasing: bool,
}

#[derive(Component)]
pub struct FireTimer {
    pub timer: Timer,
}

impl Default for FireTimer {
    fn default() -> Self {
        FireTimer {
            timer: Timer::from_seconds(FIRE_DURATION, TimerMode::Once),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum EnemyState {
    #[default]
    Spawned,
    Engaging,
    OnFire,
}

#[derive(Component)]
pub struct FollowAI;

#[derive(Component)]
pub struct ShooterAI {
    pub max_distance_from_player: f32,
    pub reload_speed: f32,
    pub reload_timer: Timer,
}
