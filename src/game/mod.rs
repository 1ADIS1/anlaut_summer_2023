mod components;
mod enemy;
mod events;
pub mod player;
mod systems;

use enemy::EnemyPlugin;
use events::{EnemyTakeDamageEvent, GameOverEvent, PlayerTakeDamageEvent};
use player::components::Player;
use player::PlayerPlugin;
use systems::*;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

const PICKUP_SPEED: f32 = 100.0;

const PICKUP_SPAWN_PERIOD: f32 = 1.0;

const PICKUP_SPRITE_SIZE: f32 = 64.0;

const FUEL_PICKUP_RESTORE: f32 = 25.0;
const HEALTH_PICKUP_RESTORE: usize = 1;

pub const CHAINSAW_FUEL_DRAIN_SPEED: f32 = 10.0;

pub const PARALLAX_SPEED: f32 = 100.0;

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

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Pickup;

#[derive(Component)]
pub struct FuelPickup;

#[derive(Component)]
pub struct HealthPickup;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            ),
            ..default()
        },
        MainCamera {},
    ));
}

// Function that spawns randowm pickups over time at the y = 0, and random x.
pub fn spawn_pickups_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    pickup_timer: Res<PickupSpawnTimer>,
) {
    let primary_window = window_query.get_single().unwrap();

    if pickup_timer.timer.just_finished() {
        spawn_fuel_bundle(&mut commands, &asset_server, primary_window);
        spawn_health_bundle(&mut commands, &asset_server, primary_window);
    }
}

pub fn spawn_fuel_bundle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    primary_window: &Window,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                random::<f32>() * primary_window.width(),
                0.0 - PICKUP_SPRITE_SIZE,
                0.0,
            ),
            texture: asset_server.load("sprites/fuel.png"),
            ..default()
        },
        Pickup {},
        FuelPickup {},
    ));
}

pub fn spawn_health_bundle(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    primary_window: &Window,
) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                random::<f32>() * primary_window.width(),
                0.0 - PICKUP_SPRITE_SIZE,
                0.0,
            ),
            texture: asset_server.load("sprites/health.png"),
            ..default()
        },
        Pickup {},
        HealthPickup {},
    ));
}

// Slowly moves all pickups from the bottom to the top of the screen
pub fn move_pickups_vertically(
    mut pickups_query: Query<&mut Transform, With<Pickup>>,
    time: Res<Time>,
) {
    // Move pickups only along y-axis
    let direction = Vec3::new(0.0, 1.0, 0.0);

    for mut pickup_transform in pickups_query.iter_mut() {
        pickup_transform.translation +=
            direction * Vec3::new(0.0, 1.0, 0.0) * PICKUP_SPEED * time.delta_seconds();
    }
}

// When pickups go off the screen, despawn them
pub fn despawn_pickups(
    mut commands: Commands,
    pickups_query: Query<(Entity, &Transform), With<Pickup>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();

    for (pickup_entity, pickup_transform) in pickups_query.iter() {
        if pickup_transform.translation.y > primary_window.height() + PICKUP_SPRITE_SIZE {
            commands.entity(pickup_entity).despawn();
        }
    }
}

// Despawn player upon game over and transition to game over state
pub fn handle_game_over_event(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    player_query: Query<Entity, With<Player>>,
) {
    for _ in game_over_event_reader.iter() {
        if let Ok(player_entity) = player_query.get_single() {
            commands.entity(player_entity).despawn();
            next_game_state.set(GameState::GAMEOVER);
            println!("Game over!");
            return;
        }
    }
}

pub fn get_cursor_world_coordinates(
    mut game_info: ResMut<GameInfo>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let primary_window = window_query.get_single().unwrap();

    if let Some(cursor_world_position) = primary_window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        game_info.cursor_position = cursor_world_position;
    }
}

pub fn tick_pickup_spawn_timer(time: Res<Time>, mut pickup_timer: ResMut<PickupSpawnTimer>) {
    pickup_timer.timer.tick(time.delta());
}
