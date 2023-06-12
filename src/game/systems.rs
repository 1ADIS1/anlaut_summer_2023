use crate::game::player::systems::change_player_fuel;

use super::components::*;
use super::enemy::components::Enemy;
use super::enemy::systems::change_enemy_health;
use super::events::*;
use super::player::components::Player;
use super::player::resources::PlayerInfo;
use super::CounterAttackTimer;
use super::{GameInfo, GameState, PickupSpawnTimer};
use super::{PARALLAX_SPEED, PICKUP_SPEED, PICKUP_SPRITE_SIZE};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_parallax_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    let bg_size = Vec2::new(320.0 * 3.0, 180.0 * 3.0);

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
            texture: asset_server.load("sprites/bg1.png"),
            sprite: Sprite {
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
        if random::<f32>() > 0.5 {
            spawn_fuel_bundle(&mut commands, &asset_server, primary_window);
        } else {
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

pub fn handle_counter_attack_event(
    mut enemy_counter_attack_event_reader: EventReader<EnemyCounterAttackEvent>,
    mut game_info: ResMut<GameInfo>,
) {
    for counter_attack_event in enemy_counter_attack_event_reader.iter() {
        game_info.counter_attack_event = counter_attack_event.clone();
        return;
    }
}

pub fn handle_counter_attack_state(
    mut game_info: ResMut<GameInfo>,
    mut counter_attack_event_failed_writer: EventWriter<CounterAttackFailed>,
    mut counter_attack_event_succeeded_writer: EventWriter<CounterAttackSucceeded>,
    counter_attack_timer: Res<CounterAttackTimer>,
    mouse_input: Res<Input<MouseButton>>,
) {
    if counter_attack_timer.timer.just_finished() {
        println!("Counter attack time has run out!");

        counter_attack_event_failed_writer.send(CounterAttackFailed {
            enemy_entity: game_info.counter_attack_event.enemy_entity,
        });
    }

    // Checks if incorrect button was pressed
    for mouse_button in vec![MouseButton::Left, MouseButton::Right] {
        if mouse_button != game_info.counter_attack_event.keys_to_press[0]
            && mouse_input.just_pressed(mouse_button)
        {
            counter_attack_event_failed_writer.send(CounterAttackFailed {
                enemy_entity: game_info.counter_attack_event.enemy_entity,
            });
        }
    }

    if mouse_input.just_pressed(game_info.counter_attack_event.keys_to_press[0]) {
        println!("One key pressed successfully!");
        game_info.counter_attack_event.keys_to_press.remove(0);

        if game_info.counter_attack_event.keys_to_press.is_empty() {
            println!("Counter attack passed successfully!");
            counter_attack_event_succeeded_writer.send(CounterAttackSucceeded {
                enemy_entity: game_info.counter_attack_event.enemy_entity,
            });
        }
    }
}

// If the player passes the counter attack:
// 1. Player kills enemy
// 2. Player gains some fuel
pub fn handle_counter_attack_succeeded_event(
    mut commands: Commands,
    mut counter_attack_event_succeeded_reader: EventReader<CounterAttackSucceeded>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut counter_attack_timer: ResMut<CounterAttackTimer>,
    mut player_info: ResMut<PlayerInfo>,
) {
    for counter_attack_succeeded_event in counter_attack_event_succeeded_reader.iter() {
        // Change state
        next_game_state.set(GameState::Running);

        // Reset timer
        counter_attack_timer.timer.reset();

        // despawn enemy
        commands
            .entity(counter_attack_succeeded_event.enemy_entity)
            .despawn();

        // Give player fuel
        let fuel_gain_amount = player_info.counter_attack_fuel_gain;
        change_player_fuel(&mut player_info, fuel_gain_amount);

        return;
    }
}

// 1. Player leaves the chainsaw mode
// 2. Player looses some fuel
// 3. Player takes damage
// 4. Enemy heals for some hp
pub fn handle_counter_attack_failed_event(
    mut counter_attack_event_failed_reader: EventReader<CounterAttackFailed>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut counter_attack_timer: ResMut<CounterAttackTimer>,
    mut enemies_query: Query<&mut Enemy>,
    mut player_info: ResMut<PlayerInfo>,
    mut player_take_damage_event_writer: EventWriter<PlayerTakeDamageEvent>,
    mut player_transition_to_regular_form_event_writer: EventWriter<
        PlayerTransitionToRegularFormEvent,
    >,
) {
    for failed_counter_attack_event in counter_attack_event_failed_reader.iter() {
        println!("Counter attack has failed!");

        // Change state
        next_game_state.set(GameState::Running);

        // Send player transition and damage events
        player_transition_to_regular_form_event_writer.send(PlayerTransitionToRegularFormEvent {});
        player_take_damage_event_writer.send(PlayerTakeDamageEvent {});

        // Reset timer
        counter_attack_timer.timer.reset();

        // Player looses fuel
        let fuel_loss_amount = player_info.counter_attack_fuel_loss;
        change_player_fuel(&mut player_info, fuel_loss_amount);

        // Heal enemy
        if let Ok(mut enemy_struct) =
            enemies_query.get_mut(failed_counter_attack_event.enemy_entity)
        {
            let heal_amount = enemy_struct.enemy_counter_attack_heal;
            change_enemy_health(&mut enemy_struct, heal_amount);
        }

        return;
    }
}

pub fn tick_pickup_spawn_timer(time: Res<Time>, mut pickup_timer: ResMut<PickupSpawnTimer>) {
    pickup_timer.timer.tick(time.delta());
}

pub fn tick_counter_attack_timer(
    mut counter_attack_timer: ResMut<CounterAttackTimer>,
    time: Res<Time>,
) {
    counter_attack_timer.timer.tick(time.delta());
}
