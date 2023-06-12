use super::components::*;
use super::resources::EnemySpawnTimer;
use super::{ENEMY_COUNTER_ATTACK_HEAL, ENEMY_COUNTER_STATE_HEALTH, ENEMY_MAX_HEALTH, ENEMY_SPEED};
use crate::game::components::Collider;
use crate::game::enemy::ENEMY_COLLIDER_SIZE;
use crate::game::events::{EnemyCounterAttackEvent, EnemyTakeDamageEvent};
use crate::game::player::components::Player;
use crate::game::player::{
    PlayerState, CHAINSAW_ENEMY_SLOW_DOWN_FACTOR, PLAYER_DAMAGE, PLAYER_DAMAGE_SPEED,
};
use crate::game::{GameState, COUNTER_ATTACK_MICE_NUMBER};

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

        let enemy_starting_position = Vec3::new(
            random::<f32>() * (primary_window.width() - ENEMY_COLLIDER_SIZE.x),
            0.0 - ENEMY_COLLIDER_SIZE.y,
            0.0,
        );
        let enemy_destination = Vec3::new(
            random::<f32>() * (primary_window.width() - ENEMY_COLLIDER_SIZE.x),
            0.0 + ENEMY_COLLIDER_SIZE.y + random::<f32>() * ENEMY_COLLIDER_SIZE.y,
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
                max_hp: ENEMY_MAX_HEALTH,
                current_hp: ENEMY_MAX_HEALTH,
                collider: Collider {
                    size: ENEMY_COLLIDER_SIZE,
                },
                counter_state_health_threshold: ENEMY_COUNTER_STATE_HEALTH,
                enemy_counter_attack_heal: ENEMY_COUNTER_ATTACK_HEAL,
                speed: ENEMY_SPEED,
                state: EnemyState::Spawned,
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
        if enemy.destination_reached && enemy.state == EnemyState::Spawned {
            enemy.state = EnemyState::Engaging;
        }

        if enemy_transform.translation.distance(enemy.destination) > 1.0 {
            enemy.destination_reached = false;
            enemy_transform.translation += enemy.direction * time.delta_seconds() * enemy.speed;
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

    for (mut enemy_transform, enemy) in enemies_query.iter_mut() {
        if enemy.state != EnemyState::Engaging {
            return;
        }

        let min_x = 0.0 + enemy.collider.size.x;
        let max_x = primary_window.width() - enemy.collider.size.x;
        let min_y = 0.0 + enemy.collider.size.y;
        let max_y = primary_window.height() - enemy.collider.size.y;

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
            if enemy.state == EnemyState::Engaging {
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
    mut enemy_counter_attack_event_writer: EventWriter<EnemyCounterAttackEvent>,
    mut enemies_query: Query<&mut Enemy>,
    mut next_game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    for enemy_damage_event in enemy_take_damage_event_reader.iter() {
        // Check if Enemy component exists on the entity from EnemyTakeDamageEvent
        // (it should definitely exists, but better to check twice)
        if let Ok(mut enemy_struct) = enemies_query.get_mut(enemy_damage_event.enemy_entity) {
            if enemy_struct.current_hp <= 0.0 {
                commands.entity(enemy_damage_event.enemy_entity).despawn();
            } else if enemy_struct.current_hp <= enemy_struct.counter_state_health_threshold {
                // Go to counter attack state
                let counter_attack_event =
                    create_enemy_counter_attack_event(enemy_damage_event.enemy_entity);
                println!("Counter Attack event sent!");
                enemy_counter_attack_event_writer.send(counter_attack_event);
                next_game_state.set(GameState::CounterAttack);
                return;
            } else {
                // Drain enemy's hp and slow it down
                enemy_struct.current_hp -=
                    PLAYER_DAMAGE as f32 * PLAYER_DAMAGE_SPEED * time.delta_seconds();
                enemy_struct.speed -= time.delta_seconds() * CHAINSAW_ENEMY_SLOW_DOWN_FACTOR;
            }
        };
    }
}

pub fn create_enemy_counter_attack_event(enemy_entity: Entity) -> EnemyCounterAttackEvent {
    let mut keys_to_press = vec![];

    for _ in 0..COUNTER_ATTACK_MICE_NUMBER {
        if random::<f32>() > 0.5 {
            keys_to_press.push(MouseButton::Left);
        } else {
            keys_to_press.push(MouseButton::Right);
        }
    }

    EnemyCounterAttackEvent {
        enemy_entity,
        keys_to_press,
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_timer: ResMut<EnemySpawnTimer>) {
    enemy_timer.timer.tick(time.delta());
}

pub fn change_enemy_health(enemy_struct: &mut Enemy, amount: f32) {
    enemy_struct.current_hp = if enemy_struct.current_hp + amount > enemy_struct.max_hp {
        enemy_struct.max_hp
    } else if enemy_struct.current_hp + amount < 0.0 {
        0.0
    } else {
        enemy_struct.current_hp + amount
    };

    println!("Enemy has healed for {}", amount);
}
