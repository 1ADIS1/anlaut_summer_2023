mod components;
pub mod enemy;
pub mod events;
pub mod player;
mod systems;

use std::collections::VecDeque;

use enemy::EnemyPlugin;
use events::*;
use player::PlayerPlugin;
use systems::*;

use bevy::prelude::*;

use self::enemy::EnemyType;

const PICKUP_SPEED: f32 = 100.0;

const PICKUP_SPAWN_PERIOD: f32 = 5.;
const HEALTH_SPAWN_CHANCE: f32 = 0.4;

const FUEL_PICKUP_SPRITE_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const FUEL_PICKUP_COLLIDER_SIZE: Vec2 = Vec2::new(64.0, 64.0);

const HEALTH_PICKUP_SPRITE_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const HEALTH_PICKUP_COLLIDER_SIZE: Vec2 = Vec2::new(64.0, 64.0);

const FUEL_PICKUP_RESTORE: f32 = 25.0;
const HEALTH_PICKUP_RESTORE: usize = 1;

pub const PARALLAX_SPEED: f32 = 1000.0;
pub const BACKGROUND_LIGHTNESS: f32 = 0.5;

pub const MAX_DEPTH: f32 = 205.0;
pub const MAX_ENEMIES_NUM: usize = 6;
pub const PLAYER_FALLING_SPEED: f32 = 1.5;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_event::<PlayerTakeDamageEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<EnemyTakeDamageEvent>()
            .add_event::<PlayerTransitionToRegularFormEvent>()
            .add_event::<ChainsawFireWave>()
            .init_resource::<GameInfo>()
            .init_resource::<PickupSpawnTimer>()
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
                    handle_projectiles,
                )
                    .in_set(OnUpdate(GameState::Running)),
            );
    }
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Running,
    Gameover,
}

// Stores every useful information for our game
#[derive(Resource, Default)]
pub struct GameInfo {
    pub cursor_position: Vec2,
    pub player_progress: f32,
    pub enemies_num: usize,
    pub enemies_spawn_queue: VecDeque<EnemyType>,
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
