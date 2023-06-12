pub mod components;
pub mod resources;
pub mod systems;

use self::resources::*;
use super::GameState;
use systems::*;

use bevy::prelude::*;

pub const PLAYER_MAX_HEALTH: usize = 5;
pub const PLAYER_FUEL_CAPACITY: f32 = 100.0;
const PLAYER_SPRITE_SIZE: f32 = 64.0;
const PLAYER_TAKE_DAMAGE_INVULNERABILITY_PERIOD: f32 = 2.0;
pub const PLAYER_DAMAGE: usize = 1;
pub const PLAYER_DAMAGE_SPEED: f32 = 30.0;
const PLAYER_REGULAR_SPEED: f32 = 225.0;
const PLAYER_CHAINSAW_SPEED: f32 = 450.0;
pub const CHAINSAW_ENEMY_SLOW_DOWN_FACTOR: f32 = 2.0;
const PLAYER_COUNTER_ATTACK_FUEL_LOSS: f32 = -25.0;
const PLAYER_COUNTER_ATTACK_FUEL_GAIN: f32 = 5.0;

const CONSTANT_PLAYER_FUEL_GAIN_AMOUNT: f32 = 1.0;
const CONSTANT_PLAYER_FUEL_GAIN_SPEED: f32 = 0.85;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PlayerState>()
            .init_resource::<PlayerInfo>()
            .init_resource::<PlayerDamageInvulnerabilityTimer>()
            .add_system(spawn_player.in_schedule(OnExit(GameState::MainMenu)))
            .add_systems(
                (
                    move_player,
                    limit_player_movement.after(move_player),
                    gain_fuel_over_time.run_if(in_state(PlayerState::REGULAR)),
                    check_player_pickup_collision,
                    transition_to_player_chainsaw_state.run_if(in_state(PlayerState::REGULAR)),
                    transition_to_player_regular_state,
                    drain_fuel.run_if(in_state(PlayerState::CHAINSAW)),
                    check_player_enemy_collision.run_if(not(in_state(PlayerState::DAMAGED))),
                    handle_player_take_damage_event.run_if(in_state(PlayerState::REGULAR)),
                    player_take_damage_invulnerability.run_if(in_state(PlayerState::DAMAGED)),
                    tick_damage_invulnerability_timer.run_if(in_state(PlayerState::DAMAGED)),
                )
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    REGULAR,
    DAMAGED,
    CHAINSAW,
}
