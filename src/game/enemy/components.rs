use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub max_hp: f32,
    pub current_hp: f32,
    pub counter_state_health_threshold: f32,
    pub enemy_counter_attack_heal: f32,
    pub speed: f32,
    pub state: EnemyState,

    pub direction: Vec3,
    pub destination: Vec3,
    pub destination_reached: bool,
}

#[derive(Debug, Default, PartialEq)]
pub enum EnemyState {
    #[default]
    Spawned,
    Engaging,
}

#[derive(Component)]
pub struct FollowAI;
