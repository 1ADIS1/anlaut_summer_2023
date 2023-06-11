mod components;
pub mod enemy;
pub mod events;
pub mod player;
mod systems;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use systems::*;

use bevy::prelude::*;

const PICKUP_SPEED: f32 = 100.0;

const PICKUP_SPAWN_PERIOD: f32 = 1.0;

const PICKUP_SPRITE_SIZE: f32 = 64.0;

const FUEL_PICKUP_RESTORE: f32 = 25.0;
const HEALTH_PICKUP_RESTORE: usize = 1;

pub const CHAINSAW_FUEL_DRAIN_SPEED: f32 = 10.0;

pub const PARALLAX_SPEED: f32 = 500.0;

pub const COUNTER_ATTACK_MICE_NUMBER: usize = 4;
pub const COUNTER_ATTACK_DURATION: f32 = 3.0;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_event::<PlayerTakeDamageEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<EnemyTakeDamageEvent>()
            .add_event::<EnemyCounterAttackEvent>()
            .add_event::<CounterAttackFailed>()
            .add_event::<PlayerTransitionToRegularFormEvent>()
            .init_resource::<GameInfo>()
            .init_resource::<PickupSpawnTimer>()
            .init_resource::<CounterAttackTimer>()
            // Run these upon start of the game
            .add_startup_system(spawn_camera)
            .add_system(spawn_parallax_background.in_schedule(OnExit(GameState::MainMenu)))
            // Run these while the game is running
            .add_systems(
                (
                    spawn_pickups_over_time,
                    move_pickups_vertically,
                    tick_pickup_spawn_timer,
                    despawn_pickups,
                    move_parallax_background,
                    handle_game_over_event,
                    get_cursor_world_coordinates,
                )
                    .in_set(OnUpdate(GameState::Running)),
            )
            // Run these when counter attack happening
            .add_system(handle_counter_attack_event.in_schedule(OnEnter(GameState::CounterAttack)))
            .add_systems(
                (
                    handle_counter_attack_state,
                    tick_counter_attack_timer,
                    handle_counter_attack_failed_event,
                )
                    .in_set(OnUpdate(GameState::CounterAttack)),
            );
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Running,
    CounterAttack,
    Gameover,
}

// Stores every useful information for our game
#[derive(Resource, Default)]
pub struct GameInfo {
    pub cursor_position: Vec2,
    pub counter_attack_event: EnemyCounterAttackEvent,
}

#[derive(Resource)]
pub struct PickupSpawnTimer {
    timer: Timer,
}

impl Default for PickupSpawnTimer {
    fn default() -> Self {
        PickupSpawnTimer {
            timer: Timer::from_seconds(PICKUP_SPAWN_PERIOD, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct CounterAttackTimer {
    timer: Timer,
}

impl Default for CounterAttackTimer {
    fn default() -> Self {
        CounterAttackTimer {
            timer: Timer::from_seconds(COUNTER_ATTACK_DURATION, TimerMode::Once),
        }
    }
}
