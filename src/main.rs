use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

const PLAYER_MAX_HEALTH: usize = 5;
const PLAYER_FUEL_CAPACITY: f32 = 100.0;

// const PLAYER_REGULAR_SPEED: f32 = 75.0;
const PICKUP_SPEED: f32 = 100.0;
const ENEMY_SPEED: f32 = 85.0;

const PICKUP_SPAWN_PERIOD: f32 = 1.0;
const ENEMY_SPAWN_PERIOD: f32 = 5.0;

const PICKUP_SPRITE_SIZE: f32 = 64.0;
const PLAYER_SPRITE_SIZE: f32 = 64.0;
const ENEMY_SPRITE_SIZE: f32 = 64.0;

const FUEL_PICKUP_RESTORE: f32 = 25.0;
const HEALTH_PICKUP_RESTORE: usize = 1;

const CHAINSAW_FUEL_DRAIN_SPEED: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<PlayerState>()
        .init_resource::<GameInfo>()
        .init_resource::<PickupSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<PlayerInfo>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(get_cursor_world_coordinates)
        .add_system(move_player)
        .add_system(limit_player_movement.after(move_player))
        .add_system(spawn_pickups_over_time)
        .add_system(move_pickups_vertically)
        .add_system(tick_pickup_spawn_timer)
        .add_system(check_player_pickup_collision)
        .add_system(despawn_pickups)
        .add_system(tick_enemy_spawn_timer)
        .add_system(spawn_enemies_over_timer)
        .add_system(move_enemies_to_arena)
        .add_system(player_info_updated)
        .add_system(transition_to_player_chainsaw_state)
        .add_system(transition_to_player_regular_state)
        .add_system(drain_fuel.run_if(in_state(PlayerState::CHAINSAW)))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerState {
    #[default]
    REGULAR,
    CHAINSAW,
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

#[derive(Resource)]
pub struct EnemySpawnTimer {
    timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer {
            timer: Timer::from_seconds(ENEMY_SPAWN_PERIOD, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Debug)]
pub struct PlayerInfo {
    pub current_fuel: f32,
    pub current_hp: usize,
    pub blood: usize,
}

impl Default for PlayerInfo {
    fn default() -> Self {
        PlayerInfo {
            current_fuel: PLAYER_FUEL_CAPACITY,
            current_hp: PLAYER_MAX_HEALTH,
            blood: 0,
        }
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Enemy {
    direction: Vec3,
    destination: Vec3,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Pickup;

#[derive(Component)]
pub struct FuelPickup;

#[derive(Component)]
pub struct HealthPickup;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {},
    ));
}

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

// Spawn enemies outside the bottom border of the screen
// And set them random direction in direction from the bottom to the arena.
pub fn spawn_enemies_over_timer(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
) {
    if enemy_timer.timer.just_finished() {
        let primary_window = window_query.get_single().unwrap();
        let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;

        let enemy_starting_position = Vec3::new(
            random::<f32>() * primary_window.width(),
            0.0 - ENEMY_SPRITE_SIZE,
            0.0,
        );
        let enemy_final_position = Vec3::new(
            random::<f32>() * primary_window.width() - enemy_radius,
            0.0 + ENEMY_SPRITE_SIZE + random::<f32>() * ENEMY_SPRITE_SIZE,
            0.0,
        );
        let enemy_direction = (enemy_final_position - enemy_starting_position).normalize();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(enemy_starting_position),
                texture: asset_server.load("sprites/enemy.png"),
                ..default()
            },
            Enemy {
                direction: enemy_direction,
                destination: enemy_final_position,
            },
        ));

        println!("Enemy has spawned, destination: {}!", enemy_final_position);
    }
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

// Before playing their AI, enemies will first slowly move to the arena outside of the screen.
pub fn move_enemies_to_arena(mut enemies_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemies_query.iter_mut() {
        // Move enemy in the direction to the final position, until it reaches it.
        if enemy_transform.translation.distance(enemy.destination) > 1.0 {
            enemy_transform.translation += enemy.direction * time.delta_seconds() * ENEMY_SPEED;
        } else {
            // Play their AI
        }
    }
}

// TODO: Plays the player chainsaw animation, sound and makes player invulnerable
// Transition to chainsaw state if the player:
// 1. In the regular form
// 2. Has maximum fuel
// 3. Clicks the LMB
pub fn transition_to_player_chainsaw_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mouse_input: Res<Input<MouseButton>>,
    player_info: Res<PlayerInfo>,
    player_query: Query<&Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok(_) = player_query.get_single() {
        if player_state.0 == PlayerState::REGULAR
            && player_info.current_fuel == PLAYER_FUEL_CAPACITY
            && mouse_input.just_pressed(MouseButton::Left)
        {
            next_player_state.set(PlayerState::CHAINSAW);
            println!("Entered chainsaw mode!");
        }
    }
}

// TODO: plays the animation, sound and shader.
pub fn transition_to_player_regular_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    player_state: Res<State<PlayerState>>,
    player_info: Res<PlayerInfo>,
) {
    if player_state.0 == PlayerState::CHAINSAW && player_info.current_fuel < 1.0 {
        next_player_state.set(PlayerState::REGULAR);
        println!("Returned back to regular form!");
    }
}

// Decrease current fuel value, while in the chainsaw state
pub fn drain_fuel(mut player_info: ResMut<PlayerInfo>, time: Res<Time>) {
    player_info.current_fuel -= CHAINSAW_FUEL_DRAIN_SPEED * time.delta_seconds();
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

pub fn check_player_pickup_collision(
    mut commands: Commands,
    mut player_info: ResMut<PlayerInfo>,
    player_query: Query<&Transform, With<Player>>,
    fuel_query: Query<(Entity, &Transform), With<FuelPickup>>,
    health_query: Query<(Entity, &Transform), With<HealthPickup>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_radius = PLAYER_SPRITE_SIZE / 2.0;
        let pickup_radius = PICKUP_SPRITE_SIZE / 2.0;

        for (fuel_entity, fuel_transform) in fuel_query.iter() {
            if player_transform
                .translation
                .distance(fuel_transform.translation)
                < player_radius + pickup_radius
            {
                // Collided with fuel
                player_info.current_fuel =
                    if PLAYER_FUEL_CAPACITY < player_info.current_fuel + FUEL_PICKUP_RESTORE {
                        PLAYER_FUEL_CAPACITY
                    } else {
                        player_info.current_fuel + FUEL_PICKUP_RESTORE
                    };
                commands.entity(fuel_entity).despawn();
            }
        }

        for (health_entity, health_transform) in health_query.iter() {
            if player_transform
                .translation
                .distance(health_transform.translation)
                < player_radius + pickup_radius
            {
                // Collided with health
                player_info.current_hp =
                    if PLAYER_MAX_HEALTH < player_info.current_hp + HEALTH_PICKUP_RESTORE {
                        PLAYER_MAX_HEALTH
                    } else {
                        player_info.current_hp + HEALTH_PICKUP_RESTORE
                    };
                commands.entity(health_entity).despawn();
            }
        }
    }
}

pub fn player_info_updated(player_info: Res<PlayerInfo>) {
    if player_info.is_changed() {
        println!("Player info: {:?}", player_info);
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

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    game_info: Res<GameInfo>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let cursor_position = game_info.cursor_position;
        player.translation = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
    }
}

pub fn limit_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let primary_window = window_query.get_single().unwrap();
        let player_radius = PLAYER_SPRITE_SIZE / 2.0;
        let x_max = primary_window.width() - player_radius;
        let x_min = 0.0 + player_radius;
        let y_max = primary_window.height() - player_radius;
        let y_min = 0.0 + player_radius;

        if player_transform.translation.x > x_max {
            player_transform.translation.x = x_max;
        }
        if player_transform.translation.x < x_min {
            player_transform.translation.x = x_min;
        }
        if player_transform.translation.y > y_max {
            player_transform.translation.y = y_max;
        }
        if player_transform.translation.y < y_min {
            player_transform.translation.y = y_min;
        }
    }
}

pub fn tick_pickup_spawn_timer(time: Res<Time>, mut pickup_timer: ResMut<PickupSpawnTimer>) {
    pickup_timer.timer.tick(time.delta());
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_timer: ResMut<EnemySpawnTimer>) {
    enemy_timer.timer.tick(time.delta());
}
