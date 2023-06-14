pub mod components;
mod resources;
pub mod systems;

use std::ops::Range;

use crate::game::GameState;
use resources::*;
use systems::*;

use bevy::prelude::*;

// === Follower ===
const FOLLOWER_MOVEMENT_SPEED: f32 = 132.6;
const FOLLOWER_HEALTH: f32 = 10.0;
const FOLLOWER_COLLIDER_SIZE: Vec2 = Vec2::new(23.0, 26.0);
const FOLLOWER_DEPTH_LEVEL: f32 = 0.0;

// === Shooter ===
const SHOOTER_MOVEMENT_SPEED: f32 = 78.0;
const SHOOTER_HEALTH: f32 = 7.5;
const SHOOTER_RELOAD_SPEED: f32 = 1.0;
const SHOOTER_DISTANCE_FROM_PLAYER: f32 = 10.0;
const SHOOTER_COLLIDER_SIZE: Vec2 = Vec2::new(62.0, 57.0);
const SHOOTER_DEPTH_LEVEL: f32 = 50.0;
const SHOOTER_PROJECTILE_SPEED: f32 = 180.0;
const SHOOTER_PROJECTILE_SIZE: Vec2 = Vec2::new(39.2, 39.2);
const SHOOTER_PROJECTILE_COLLIDER_SIZE: Vec2 = Vec2::new(32., 32.);

const FIRE_DURATION: f32 = 4.0;
const ENEMY_ON_FIRE_SPEED_GAIN: f32 = 50.0;
const FIRE_COLOR_SPEED: f32 = 17.5;
const FIRE_FLASH_GREEN_MIN: f32 = 100.0;
const FIRE_FLASH_GREEN_MAX: f32 = 200.0;

const ENEMY_SPAWN_PERIOD: f32 = 3.5;
const ENEMY_RANGE_SPEED: Range<f32> = 0.85..1.;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>().add_systems(
            (
                tick_enemy_spawn_timer,
                tick_enemy_fire_timer,
                tick_shooter_reloading_timer,
                handle_on_fire_state,
                spawn_enemies_over_time,
                handle_enemy_take_damage_event,
                move_enemies_to_destination,
                follow_player,
                handle_shooter_ai,
                handle_fire_wave_event,
                limit_enemy_movement.after(move_enemies_to_destination),
            )
                .in_set(OnUpdate(GameState::Running)),
        );
    }
}

#[derive(PartialEq)]
pub enum EnemyType {
    Follower,
    Shooter,
}
