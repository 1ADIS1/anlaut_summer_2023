use super::components::*;
use super::resources::EnemySpawnTimer;
use super::*;
use crate::game::components::{Collider, Projectile};
use crate::game::events::{ChainsawFireWave, EnemyTakeDamageEvent};
use crate::game::player::components::Player;
use crate::game::player::{
    PlayerState, CHAINSAW_ENEMY_SLOW_DOWN_FACTOR, PLAYER_CHAINSAW_COLLIDER_SIZE, PLAYER_DAMAGE,
    PLAYER_DAMAGE_SPEED,
};
use crate::game::{GameInfo, MAX_ENEMIES_NUM};

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

// TODO: fix bug when enemy can leave borders of the window when running from the player

// Spawn enemies outside the bottom border of the screen
// And set them random direction in direction from the bottom to the arena.
pub fn spawn_enemies_over_time(
    mut commands: Commands,
    mut game_info: ResMut<GameInfo>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_timer: Res<EnemySpawnTimer>,
) {
    if enemy_timer.timer.just_finished() {
        if game_info.enemies_num >= MAX_ENEMIES_NUM || game_info.is_boss_spawned {
            return;
        }

        let primary_window = window_query.get_single().unwrap();

        let mut rng = thread_rng();

        let player_progress = game_info.player_progress;
        let enemies_num = game_info.enemies_num;

        game_info.enemies_num += 1;
        game_info
            .enemies_spawn_queue
            .push_back(if player_progress >= BOSS_DEPTH_LEVEL {
                EnemyType::Boss
            } else if player_progress >= SHOOTER_DEPTH_LEVEL {
                if rng.gen::<f32>() > 0.4 {
                    EnemyType::Shooter
                } else {
                    EnemyType::Follower
                }
            } else {
                EnemyType::Follower
            });

        if game_info.enemies_spawn_queue.back() == Some(&EnemyType::Boss) {
            game_info.is_boss_spawned = true;
        }

        // === Shooter ===
        if game_info.enemies_spawn_queue.front().unwrap() == &EnemyType::Shooter {
            let speed: f32 = rng.gen_range(ENEMY_RANGE_SPEED) * SHOOTER_MOVEMENT_SPEED;

            let enemy_starting_position = Vec3::new(
                rng.gen::<f32>() * (primary_window.width() - SHOOTER_COLLIDER_SIZE.x),
                0.0 - SHOOTER_COLLIDER_SIZE.y,
                0.0,
            );
            let enemy_destination = Vec3::new(
                rng.gen::<f32>() * (primary_window.width() - SHOOTER_COLLIDER_SIZE.x),
                0.0 + SHOOTER_COLLIDER_SIZE.y + rng.gen::<f32>() * SHOOTER_COLLIDER_SIZE.y,
                0.0,
            );
            let enemy_direction = (enemy_destination - enemy_starting_position).normalize();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(enemy_starting_position),
                    texture: asset_server.load("sprites/shooter_default.png"),
                    ..default()
                },
                Enemy {
                    max_hp: SHOOTER_HEALTH,
                    current_hp: SHOOTER_HEALTH,

                    current_speed: speed,
                    default_speed: speed,

                    enemy_type: EnemyType::Shooter,
                    depth_level: SHOOTER_DEPTH_LEVEL,

                    collider: Collider {
                        size: SHOOTER_COLLIDER_SIZE,
                    },
                    state: EnemyState::Spawned,

                    direction: enemy_direction,
                    destination: enemy_destination,

                    destination_reached: false,
                    is_green_decreasing: false,
                },
                FireTimer::default(),
                ShooterAI {
                    reload_speed: SHOOTER_RELOAD_SPEED,
                    max_distance_from_player: SHOOTER_DISTANCE_FROM_PLAYER,
                    reload_timer: Timer::from_seconds(SHOOTER_RELOAD_SPEED, TimerMode::Once),
                },
            ));
        }
        // === Follower ===
        else if game_info.enemies_spawn_queue.front().unwrap() == &EnemyType::Follower {
            let speed: f32 = rng.gen_range(ENEMY_RANGE_SPEED) * FOLLOWER_MOVEMENT_SPEED;

            let enemy_starting_position = Vec3::new(
                rng.gen::<f32>() * (primary_window.width() - FOLLOWER_COLLIDER_SIZE.x),
                0.0 - FOLLOWER_COLLIDER_SIZE.y,
                0.0,
            );
            let enemy_destination = Vec3::new(
                rng.gen::<f32>() * (primary_window.width() - FOLLOWER_COLLIDER_SIZE.x),
                0.0 + FOLLOWER_COLLIDER_SIZE.y + rng.gen::<f32>() * FOLLOWER_COLLIDER_SIZE.y,
                0.0,
            );
            let enemy_direction = (enemy_destination - enemy_starting_position).normalize();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(enemy_starting_position),
                    texture: asset_server.load("sprites/follower_default.png"),
                    ..default()
                },
                Enemy {
                    max_hp: FOLLOWER_HEALTH,
                    current_hp: FOLLOWER_HEALTH,

                    current_speed: speed,
                    default_speed: speed,

                    enemy_type: EnemyType::Follower,
                    depth_level: FOLLOWER_DEPTH_LEVEL,

                    collider: Collider {
                        size: FOLLOWER_COLLIDER_SIZE,
                    },
                    state: EnemyState::Spawned,

                    direction: enemy_direction,
                    destination: enemy_destination,

                    destination_reached: false,
                    is_green_decreasing: false,
                },
                FireTimer::default(),
                FollowAI {},
            ));
        }
        // === Boss ===
        else if game_info.enemies_spawn_queue.front().unwrap() == &EnemyType::Boss {
            let speed: f32 = rng.gen_range(ENEMY_RANGE_SPEED) * BOSS_MOVEMENT_SPEED;

            let enemy_starting_position = Vec3::new(
                primary_window.width() / 2.0,
                0.0 - BOSS_COLLIDER_SIZE.y,
                0.0,
            );
            let enemy_destination = Vec3::new(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            );
            let enemy_direction = (enemy_destination - enemy_starting_position).normalize();

            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_translation(enemy_starting_position),
                    texture: asset_server.load("sprites/shooter_default.png"),
                    sprite: Sprite {
                        custom_size: Some(BOSS_SPRITE_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Enemy {
                    max_hp: BOSS_HEALTH,
                    current_hp: BOSS_HEALTH,

                    current_speed: speed,
                    default_speed: speed,

                    enemy_type: EnemyType::Boss,
                    depth_level: BOSS_DEPTH_LEVEL,

                    collider: Collider {
                        size: BOSS_COLLIDER_SIZE,
                    },
                    state: EnemyState::Spawned,

                    direction: enemy_direction,
                    destination: enemy_destination,

                    destination_reached: false,
                    is_green_decreasing: false,
                },
                FireTimer::default(),
                ShooterAI {
                    max_distance_from_player: BOSS_DISTANCE_FROM_PLAYER,
                    reload_speed: BOSS_RELOAD_SPEED,
                    reload_timer: Timer::from_seconds(BOSS_RELOAD_SPEED, TimerMode::Once),
                },
            ));
        }

        game_info.enemies_spawn_queue.pop_front();
    }
}

// Upon spawning, enemies will slowly move to the arena outside of the screen.
// When they reach the destination, enemy transitions to ENGAGING state.
// Move enemies to destination using their direction.
// When they reach destination, set destination_reached to true.
pub fn move_enemies_to_destination(
    mut enemies_query: Query<(&mut Transform, &mut Enemy)>,
    player_state: Res<State<PlayerState>>,
    time: Res<Time>,
) {
    for (mut enemy_transform, mut enemy_struct) in enemies_query.iter_mut() {
        if enemy_struct.destination_reached && enemy_struct.state == EnemyState::Spawned {
            enemy_struct.state = EnemyState::Engaging;
        }

        if enemy_struct.state == EnemyState::OnFire {
            enemy_struct.destination_reached = false;
            enemy_transform.translation +=
                enemy_struct.direction * time.delta_seconds() * enemy_struct.current_speed;
            continue;
        }

        if enemy_struct.enemy_type == EnemyType::Follower && player_state.0 == PlayerState::CHAINSAW
        {
            enemy_struct.direction =
                (enemy_transform.translation - enemy_struct.destination).normalize();
        } else {
            enemy_struct.direction =
                (enemy_struct.destination - enemy_transform.translation).normalize();
        }

        if enemy_transform
            .translation
            .distance(enemy_struct.destination)
            > 1.0
        {
            enemy_struct.destination_reached = false;
            enemy_transform.translation +=
                enemy_struct.direction * time.delta_seconds() * enemy_struct.current_speed;
        } else {
            enemy_struct.destination_reached = true;
        }
    }
}

// Prevents enemies from going outside of the window in the engagement or on fire states
pub fn limit_enemy_movement(
    mut enemies_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary_window = window_query.get_single().unwrap();

    for (mut enemy_transform, mut enemy_struct) in enemies_query.iter_mut() {
        if enemy_struct.state == EnemyState::Spawned {
            continue;
        }

        let min_x = 0.0 + PLAYER_CHAINSAW_COLLIDER_SIZE.x;
        let max_x = primary_window.width() - PLAYER_CHAINSAW_COLLIDER_SIZE.x;
        let min_y = 0.0 + PLAYER_CHAINSAW_COLLIDER_SIZE.y;
        let max_y = primary_window.height() - PLAYER_CHAINSAW_COLLIDER_SIZE.y;

        if enemy_struct.state == EnemyState::OnFire {
            if enemy_transform.translation.x < min_x || enemy_transform.translation.x > max_x {
                enemy_struct.direction.x *= -1.0;
            }
            if enemy_transform.translation.y < min_y || enemy_transform.translation.y > max_y {
                enemy_struct.direction.y *= -1.0;
            }
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
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (_, mut enemy_struct) in enemies_query.iter_mut() {
            if enemy_struct.state != EnemyState::Engaging {
                continue;
            }

            enemy_struct.destination = player_transform.translation;
            // enemy_struct.direction = if player_state.0 == PlayerState::CHAINSAW {
            //     (enemy_transform.translation - player_transform.translation).normalize()
            // } else {
            //     (player_transform.translation - enemy_transform.translation).normalize()
            // };
        }
    }
}

pub fn handle_shooter_ai(
    mut commands: Commands,
    mut enemies_query: Query<(&Transform, &mut Enemy, &mut ShooterAI)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    if let Ok(player_transform) = player_query.get_single() {
        for (enemy_transform, mut enemy_struct, mut shooter_struct) in enemies_query.iter_mut() {
            if enemy_struct.state != EnemyState::Engaging {
                continue;
            }

            let max_x = primary_window.width() - enemy_struct.collider.size.x;
            let max_y = primary_window.height() - enemy_struct.collider.size.y;

            if enemy_struct.destination_reached {
                enemy_struct.destination =
                    Vec3::new(random::<f32>() * max_x, random::<f32>() * max_y, 0.0);
            }

            // shoot projectile
            if shooter_struct.reload_timer.just_finished() {
                if enemy_struct.enemy_type == EnemyType::Boss {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(enemy_transform.translation),
                            texture: asset_server.load("sprites/projectile.png"),
                            sprite: Sprite {
                                custom_size: Some(BOSS_PROJECTILE_SIZE),
                                ..default()
                            },
                            ..default()
                        },
                        Projectile {
                            speed: BOSS_PROJECTILE_SPEED,
                            direction: (player_transform.translation - enemy_transform.translation)
                                .normalize(),
                            collider: Collider {
                                size: BOSS_PROJECTILE_COLLIDER_SIZE,
                            },
                        },
                    ));
                } else {
                    commands.spawn((
                        SpriteBundle {
                            transform: Transform::from_translation(enemy_transform.translation),
                            texture: asset_server.load("sprites/projectile.png"),
                            sprite: Sprite {
                                custom_size: Some(SHOOTER_PROJECTILE_SIZE),
                                ..default()
                            },
                            ..default()
                        },
                        Projectile {
                            speed: SHOOTER_PROJECTILE_SPEED,
                            direction: (player_transform.translation - enemy_transform.translation)
                                .normalize(),
                            collider: Collider {
                                size: SHOOTER_PROJECTILE_COLLIDER_SIZE,
                            },
                        },
                    ));
                }
                shooter_struct.reload_timer.reset();
            }
        }
    }
}

pub fn handle_enemy_take_damage_event(
    mut commands: Commands,
    mut enemy_take_damage_event_reader: EventReader<EnemyTakeDamageEvent>,
    mut enemies_query: Query<&mut Enemy>,
    mut game_info: ResMut<GameInfo>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for enemy_damage_event in enemy_take_damage_event_reader.iter() {
        // Check if Enemy component exists on the entity from EnemyTakeDamageEvent
        // (it should definitely exists, but better to check twice)
        if let Ok(mut enemy_struct) = enemies_query.get_mut(enemy_damage_event.enemy_entity) {
            if enemy_struct.current_hp <= 0.0 {
                let enemies_num = game_info.enemies_num;
                game_info.enemies_num = if enemies_num != 0 { enemies_num - 1 } else { 0 };

                if enemy_struct.enemy_type == EnemyType::Boss {
                    let primary_window = window_query.get_single().unwrap();

                    println!("Spawned end screen");

                    commands.spawn(SpriteBundle {
                        transform: Transform::from_xyz(
                            primary_window.width() / 2.,
                            primary_window.height() / 2.,
                            0.0,
                        ),
                        texture: asset_server.load("sprites/end_screen.png"),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(260. * 2., 320. * 2.)),
                            ..default()
                        },
                        ..default()
                    });
                }

                commands.entity(enemy_damage_event.enemy_entity).despawn();
            } else {
                // Drain enemy's hp and slow it down
                enemy_struct.current_hp -=
                    PLAYER_DAMAGE as f32 * PLAYER_DAMAGE_SPEED * time.delta_seconds();
                enemy_struct.current_speed -=
                    time.delta_seconds() * CHAINSAW_ENEMY_SLOW_DOWN_FACTOR;
            }
        };
    }
}

// Enemy becomes ignited and changes his AI
pub fn handle_fire_wave_event(
    mut fire_wave_event_reader: EventReader<ChainsawFireWave>,
    mut enemies_query: Query<(&mut Enemy, &mut FireTimer)>,
) {
    for _ in fire_wave_event_reader.iter() {
        for (mut enemy_struct, mut enemy_fire_timer) in enemies_query.iter_mut() {
            if enemy_struct.state != EnemyState::Engaging {
                continue;
            }

            enemy_struct.state = EnemyState::OnFire;
            enemy_struct.current_speed += ENEMY_ON_FIRE_SPEED_GAIN;
            enemy_struct.is_green_decreasing = true;
            enemy_fire_timer.timer.reset();
        }

        return;
    }
}

pub fn handle_on_fire_state(
    mut enemies_query: Query<(&mut Sprite, &mut Enemy, &FireTimer)>,
    time: Res<Time>,
) {
    for (mut enemy_sprite, mut enemy_struct, enemy_fire_timer) in enemies_query.iter_mut() {
        if enemy_struct.state != EnemyState::OnFire {
            continue;
        }

        if enemy_fire_timer.timer.just_finished() {
            enemy_struct.state = EnemyState::Engaging;
            enemy_struct.current_speed -= ENEMY_ON_FIRE_SPEED_GAIN;
            enemy_sprite.color = Color::WHITE;
            enemy_struct.is_green_decreasing = false;
            enemy_struct.destination_reached = false;

            println!("Enemy has extinguished the fire");
            continue;
        }

        // Periodical orange flash
        let mut enemy_color = enemy_sprite.color;

        if enemy_color.g() * 255.0 >= FIRE_FLASH_GREEN_MAX {
            enemy_struct.is_green_decreasing = true;
        } else if enemy_color.g() * 255.0 <= FIRE_FLASH_GREEN_MIN {
            enemy_struct.is_green_decreasing = false;
        }

        if enemy_struct.is_green_decreasing {
            enemy_color = Color::rgb_linear(
                enemy_color.r(),
                enemy_color.g() - FIRE_COLOR_SPEED * time.delta_seconds(),
                enemy_color.b() - FIRE_COLOR_SPEED * time.delta_seconds(),
            );

            println!("Enemy's color is decreasing");
        } else {
            enemy_color = Color::rgb_linear(
                enemy_color.r(),
                enemy_color.g() + FIRE_COLOR_SPEED * time.delta_seconds(),
                enemy_color.b() + FIRE_COLOR_SPEED * time.delta_seconds(),
            );

            println!("Enemy's color is increasing");
        }

        enemy_sprite.color = enemy_color;
    }
}

pub fn tick_enemy_spawn_timer(time: Res<Time>, mut enemy_spawn_timer: ResMut<EnemySpawnTimer>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

// Tick the timer only when the enemy is on fire
pub fn tick_enemy_fire_timer(mut enemy_queries: Query<(&mut FireTimer, &Enemy)>, time: Res<Time>) {
    for (mut enemy_fire_timer, enemy_struct) in enemy_queries.iter_mut() {
        if enemy_struct.state == EnemyState::OnFire {
            enemy_fire_timer.timer.tick(time.delta());
        }
    }
}

pub fn tick_shooter_reloading_timer(
    mut enemy_queries: Query<(&mut ShooterAI, &Enemy)>,
    time: Res<Time>,
) {
    for (mut shooter_struct, enemy_struct) in enemy_queries.iter_mut() {
        if enemy_struct.state != EnemyState::Engaging {
            continue;
        }

        shooter_struct.reload_timer.tick(time.delta());
    }
}

#[allow(dead_code)]
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
