use super::components::*;
use super::resources::EnemySpawnTimer;
use super::{ENEMY_MAX_HEALTH, ENEMY_SPEED, ENEMY_SPRITE_SIZE};
use crate::game::events::EnemyTakeDamageEvent;
use crate::game::player::{PLAYER_DAMAGE, PLAYER_DAMAGE_SPEED};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

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
                current_hp: ENEMY_MAX_HEALTH,
                direction: enemy_direction,
                destination: enemy_final_position,
            },
        ));

        println!("Enemy has spawned, destination: {}!", enemy_final_position);
    }
}

// Before playing their AI, enemies will first slowly move to the arena outside of the screen.
pub fn move_enemies_to_arena(mut enemies_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut enemy_transform, enemy) in enemies_query.iter_mut() {
        // Move enemy in the direction to the final position, until it reaches it.
        if enemy_transform.translation.distance(enemy.destination) > 1.0 {
            enemy_transform.translation += enemy.direction * time.delta_seconds() * ENEMY_SPEED;
        } else {
            // Play enemy AI
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
