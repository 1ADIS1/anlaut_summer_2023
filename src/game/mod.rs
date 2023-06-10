mod components;
mod enemy;
mod events;
pub mod player;
mod systems;

use enemy::EnemyPlugin;
use events::{EnemyTakeDamageEvent, GameOverEvent, PlayerTakeDamageEvent};
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_state::<GameState>()
            .add_event::<PlayerTakeDamageEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<EnemyTakeDamageEvent>()
            .init_resource::<GameInfo>()
            .init_resource::<PickupSpawnTimer>()
            .add_systems(
                (spawn_camera, spawn_parallax_background).in_schedule(OnEnter(GameState::RUNNING)),
            )
            .add_systems((
                get_cursor_world_coordinates,
                spawn_pickups_over_time,
                move_pickups_vertically,
                tick_pickup_spawn_timer,
                despawn_pickups,
                handle_game_over_event,
                move_parallax_background,
            ));
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    RUNNING,
    GAMEOVER,
}

// Stores every useful information for our game
#[derive(Resource, Default)]
pub struct GameInfo {
    cursor_position: Vec2,
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
