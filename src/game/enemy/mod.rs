pub mod components;
mod resources;
pub mod systems;

use crate::game::GameState;
use resources::*;
use systems::*;

use bevy::prelude::*;

const ENEMY_SPEED: f32 = 85.0;
const ENEMY_SPAWN_PERIOD: f32 = 3.0;

pub const ENEMY_SPRITE_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const ENEMY_COLLIDER_SIZE: Vec2 = Vec2::new(64.0, 64.0);

const ENEMY_MAX_HEALTH: f32 = 10.0;
// When does enemy goes to counter attack state
const ENEMY_COUNTER_STATE_HEALTH: f32 = 5.0;
const ENEMY_COUNTER_ATTACK_HEAL: f32 = 25.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>().add_systems(
            (
                tick_enemy_spawn_timer,
                spawn_enemies_over_timer,
                handle_enemy_take_damage_event,
                move_enemies_to_destination,
                follow_player,
                limit_enemy_movement_in_engaging_state.after(move_enemies_to_destination),
            )
                .in_set(OnUpdate(GameState::Running)),
        );
    }
}
