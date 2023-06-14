use super::components::*;
use super::events::*;
use super::player::components::Player;
use super::BACKGROUND_LIGHTNESS;
use super::HEALTH_SPAWN_CHANCE;
use super::{GameInfo, GameState, PickupSpawnTimer};
use super::{
    FUEL_PICKUP_COLLIDER_SIZE, FUEL_PICKUP_SPRITE_SIZE, HEALTH_PICKUP_COLLIDER_SIZE,
    HEALTH_PICKUP_SPRITE_SIZE, PARALLAX_SPEED, PICKUP_SPEED,
};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_parallax_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    let bg_size = Vec2::new(260.0 * 2.0, 320.0 * 2.0);

    // Top background. Spawn it at the center of the screen.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                -1.0,
            ),
            texture: asset_server.load("sprites/bg1.png"),
            sprite: Sprite {
                color: Color::Hsla {
                    hue: 360.,
                    saturation: 0.,
                    lightness: BACKGROUND_LIGHTNESS,
                    alpha: 1.,
                },
                custom_size: Some(bg_size),
                ..default()
            },
            ..default()
        },
        ParallaxBackground { size: bg_size },
    ));

    // Bottom background. Spawn it outside of the screen.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                -(primary_window.height() / 2.0),
                -1.0,
            ),
            texture: asset_server.load("sprites/bg2.png"),
            sprite: Sprite {
                color: Color::Hsla {
                    hue: 360.,
                    saturation: 0.,
                    lightness: BACKGROUND_LIGHTNESS,
                    alpha: 1.,
                },
                custom_size: Some(bg_size),
                ..default()
            },
            ..default()
        },
        ParallaxBackground { size: bg_size },
    ));
}

// Moves background up
// When goes out of the bounds -> move it at the bottom of the screen and move it for one additional frame more
// TODO: fix overheads
pub fn move_parallax_background(
    mut parallax_background_query: Query<(&mut Transform, &ParallaxBackground)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let primary_window = window_query.get_single().unwrap();
    let parallax_direction = Vec3::new(0.0, 1.0, 0.0).normalize();

    for (mut parallax_bg_transform, _) in parallax_background_query.iter_mut() {
        parallax_bg_transform.translation +=
            parallax_direction * PARALLAX_SPEED * time.delta_seconds();

        if parallax_bg_transform.translation.y - primary_window.height() / 2.0
            > primary_window.height()
        {
            parallax_bg_transform.translation.y -= 2.0 * primary_window.height();
        }
    }
}

pub fn handle_projectiles(
    mut commands: Commands,
    mut projectiles_query: Query<(Entity, &mut Transform, &Projectile)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let primary_window = window_query.get_single().unwrap();

    for (entity, mut projectile_transform, projectile_struct) in projectiles_query.iter_mut() {
        let min_x = 0.0 + projectile_struct.collider.size.x;
        let max_x = primary_window.width() - projectile_struct.collider.size.x;
        let min_y = 0.0 + projectile_struct.collider.size.y;
        let max_y = primary_window.height() - projectile_struct.collider.size.y;

        projectile_transform.translation +=
            projectile_struct.direction * projectile_struct.speed * time.delta_seconds();

        if projectile_transform.translation.x < min_x
            || projectile_transform.translation.x > max_x
            || projectile_transform.translation.y < min_y
            || projectile_transform.translation.y > max_y
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    // commands.spawn((
    //     Camera2dBundle {
    //         transform: Transform::from_xyz(
    //             primary_window.width() / 2.0,
    //             primary_window.height() / 2.0,
    //             0.0,
    //         ),
    //         ..default()
    //     },
    //     MainCamera {},
    // ));

    commands.spawn({
        let bundle = (
            Camera2dBundle {
                transform: Transform::from_xyz(
                    primary_window.width() / 2.0,
                    primary_window.height() / 2.0,
                    0.0,
                ),
                ..default()
            },
            MainCamera {},
        );
        // bundle.0.projection.scaling_mode =
        //     bevy::render::camera::ScalingMode::FixedHorizontal(260.0);
        // bundle.0.projection.scaling_mode = bevy::render::camera::ScalingMode::WindowSize(1.0);
        // bundle.0.transform.scale = Vec3::new(1., 1., 1.);
        bundle
    });
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

        if random::<f32>() > HEALTH_SPAWN_CHANCE {
            spawn_health_bundle(&mut commands, &asset_server, primary_window);
        }
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
                random::<f32>() * (primary_window.width() - FUEL_PICKUP_SPRITE_SIZE.x),
                0.0 - FUEL_PICKUP_SPRITE_SIZE.y,
                0.0,
            ),
            texture: asset_server.load("sprites/fuel.png"),
            ..default()
        },
        Pickup {
            collider: Collider {
                size: FUEL_PICKUP_COLLIDER_SIZE,
            },
        },
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
                random::<f32>() * (primary_window.width() - HEALTH_PICKUP_SPRITE_SIZE.x),
                0.0 - HEALTH_PICKUP_SPRITE_SIZE.y,
                0.0,
            ),
            texture: asset_server.load("sprites/health.png"),
            ..default()
        },
        Pickup {
            collider: Collider {
                size: HEALTH_PICKUP_COLLIDER_SIZE,
            },
        },
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
    pickups_query: Query<(Entity, &Transform, &Pickup)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();

    for (pickup_entity, pickup_transform, pickup_struct) in pickups_query.iter() {
        if pickup_transform.translation.y > primary_window.height() + pickup_struct.collider.size.y
        {
            commands.entity(pickup_entity).despawn();
        }
    }
}

// Play fire shader
// pub fn handle_firewave_event() {}

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
            next_game_state.set(GameState::Gameover);
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
