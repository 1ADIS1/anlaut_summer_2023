use super::components::*;
use super::resources::EnemySpawnTimer;
use super::{ENEMY_MAX_HEALTH, ENEMY_SPEED, ENEMY_SPRITE_SIZE};
use crate::game::events::EnemyTakeDamageEvent;
use crate::game::player::components::Player;
use crate::game::player::{PlayerState, PLAYER_DAMAGE, PLAYER_DAMAGE_SPEED};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

// TODO: fix bug when enemy can leave borders of the window when running from the player

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
            random::<f32>() * (primary_window.width() - enemy_radius),
            0.0 - enemy_radius,
            0.0,
        );
        let enemy_destination = Vec3::new(
            random::<f32>() * (primary_window.width() - enemy_radius),
            0.0 + enemy_radius + random::<f32>() * enemy_radius,
            0.0,
        );
        let enemy_direction = (enemy_destination - enemy_starting_position).normalize();

        // TODO: Spawn different enemies
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(enemy_starting_position),
                texture: asset_server.load("sprites/enemy.png"),
                ..default()
            },
            Enemy {
                current_hp: ENEMY_MAX_HEALTH,
                speed: ENEMY_SPEED,
                state: EnemyState::SPAWNED,
                direction: enemy_direction,
                destination: enemy_destination,
                destination_reached: false,
            },
            FollowAI {},
        ));

        println!("Enemy has spawned, destination: {}!", enemy_destination);
    }
}

// Upon spawning, enemies will slowly move to the arena outside of the screen.
// When they reach the destination, enemy transitions to ENGAGING state.
// Move enemies to destination using their direction.
// When they reach destination, set destination_reached to true.
pub fn move_enemies_to_destination(
    mut enemies_query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    for (mut enemy_transform, mut enemy) in enemies_query.iter_mut() {
        if enemy.destination_reached && enemy.state == EnemyState::SPAWNED {
            enemy.state = EnemyState::ENGAGING;
        }

        if enemy_transform.translation.distance(enemy.destination) > 1.0 {
            enemy.destination_reached = false;
            enemy_transform.translation += enemy.direction * time.delta_seconds() * ENEMY_SPEED;
        } else {
            enemy.destination_reached = true;
        }
    }
}

// Prevents enemies from going outside of the window in the Engagement state.
pub fn limit_enemy_movement_in_engaging_state(
    mut enemies_query: Query<(&mut Transform, &Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();
    let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;
    let min_x = 0.0 + enemy_radius;
    let max_x = primary_window.width() - enemy_radius;
    let min_y = 0.0 + enemy_radius;
    let max_y = primary_window.height() - enemy_radius;

    for (mut enemy_transform, enemy) in enemies_query.iter_mut() {
        if enemy.state != EnemyState::ENGAGING {
            return;
        }

        if enemy_transform.translation.x < min_x {
            enemy_transform.translation.x = min_x;
        }
        if enemy_transform.translation.x > max_x {
            enemy_transform.translation.x = max_x;
        }
        if enemy_transform.translation.y < min_y {
            enemy_transform.translation.y = min_y;
        }
        if enemy_transform.translation.y > max_y {
            enemy_transform.translation.y = max_y;
        }
    }
}

// Moves all enemies with FollowAI to player, if they have appeared on the arena.
// If the player is in the chainsaw mode, run from him in the opposite direction.
pub fn follow_player(
    mut enemies_query: Query<(&Transform, &mut Enemy), With<FollowAI>>,
    player_query: Query<&Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut enemy) in enemies_query.iter_mut() {
            if enemy.state == EnemyState::ENGAGING {
                enemy.destination = player_transform.translation;
                enemy.direction = if player_state.0 == PlayerState::CHAINSAW {
                    (enemy_transform.translation - player_transform.translation).normalize()
                } else {
                    (player_transform.translation - enemy_transform.translation).normalize()
                };
            }
        }
    }
}

pub fn handle_enemy_take_damage_event(
    mut commands: Commands,
    mut enemy_take_damage_event_reader: EventReader<EnemyTakeDamageEvent>,
    mut enemies_query: Query<&mut Enemy>,
    time: Res<Time>,
) {
    for enemy_damage_event in enemy_take_damage_event_reader.iter() {
        // Check if Enemy component exists on the entity from EnemyTakeDamageEvent
        // (it should definitely exists, but better to check twice)
        if let Ok(mut enemy_struct) = enemies_query.get_mut(enemy_damage_event.enemy_entity) {
            if enemy_struct.current_hp <= 0.0 {
                commands.entity(enemy_damage_event.enemy_entity).despawn();
            } else {
                // Drain enemy's hp
                enemy_struct.current_hp -=
                    PLAYER_DAMAGE as f32 * PLAYER_DAMAGE_SPEED * time.delta_seconds();
            }
        };
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_timer: ResMut<EnemySpawnTimer>) {
    enemy_timer.timer.tick(time.delta());
}
