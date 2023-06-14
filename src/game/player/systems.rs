use super::components::Player;
use super::resources::PlayerDamageInvulnerabilityTimer;
use super::{
    PlayerInfo, PlayerState, CHAINSAW_HEAT_LIMIT, CHAINSAW_HEAT_SPEED,
    PLAYER_CHAINSAW_COLLIDER_SIZE,
};
use super::{
    CHAINSAW_FUEL_DRAIN_SPEED, PASSIVE_PLAYER_FUEL_GAIN_AMOUNT, PASSIVE_PLAYER_FUEL_GAIN_SPEED,
    PLAYER_CHAINSAW_SPEED, PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH, PLAYER_REGULAR_COLLIDER_SIZE,
    PLAYER_REGULAR_SPEED,
};
use crate::game::components::{Collider, FuelPickup, HealthPickup, Pickup, Projectile};
use crate::game::enemy::components::Enemy;
use crate::game::events::{
    ChainsawFireWave, EnemyTakeDamageEvent, GameOverEvent, PlayerTakeDamageEvent,
    PlayerTransitionToRegularFormEvent,
};
use crate::game::{GameInfo, MAX_DEPTH, PLAYER_FALLING_SPEED};
use crate::game::{FUEL_PICKUP_RESTORE, HEALTH_PICKUP_RESTORE};

use bevy::prelude::*;
use bevy::sprite::collide_aabb::*;
use bevy::window::PrimaryWindow;

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
            texture: asset_server.load("sprites/player_falling.png"),
            ..default()
        },
        Player {
            current_speed: PLAYER_REGULAR_SPEED,
            collider: Collider {
                size: PLAYER_REGULAR_COLLIDER_SIZE,
            },
        },
    ));
}

// TODO: Plays the player chainsaw animation, sound and makes player invulnerable
// Transition to chainsaw state if the player:
// 1. In the regular form
// 2. Has maximum fuel
// 3. Clicks the LMB
pub fn transition_to_player_chainsaw_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<(&mut Handle<Image>, &mut Player)>,
    mouse_input: Res<Input<MouseButton>>,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut player_texture, mut player)) = player_query.get_single_mut() {
        if player_info.current_fuel == PLAYER_FUEL_CAPACITY
            && mouse_input.just_pressed(MouseButton::Left)
        {
            next_player_state.set(PlayerState::CHAINSAW);
            player.current_speed = PLAYER_CHAINSAW_SPEED;

            *player_texture = asset_server.load("sprites/player_chainsaw.png");
            player.collider.size = PLAYER_CHAINSAW_COLLIDER_SIZE;
        }
    }
}

// TODO: plays the animation, sound and shader.
pub fn transition_to_player_regular_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<(&mut Handle<Image>, &mut Sprite, &mut Player)>,
    mut player_transition_to_regular_form_event_reader: EventReader<
        PlayerTransitionToRegularFormEvent,
    >,
    mut player_info: ResMut<PlayerInfo>,
    asset_server: Res<AssetServer>,
) {
    for _ in player_transition_to_regular_form_event_reader.iter() {
        if let Ok((mut player_texture, mut player_sprite, mut player)) =
            player_query.get_single_mut()
        {
            next_player_state.set(PlayerState::DAMAGED);
            player.current_speed = PLAYER_REGULAR_SPEED;
            player_info.chainsaw_heat = 0.0;
            player_sprite.color = Color::WHITE;

            *player_texture = asset_server.load("sprites/player_falling.png");
            player.collider.size = PLAYER_REGULAR_COLLIDER_SIZE;
        }
    }
}

// Decrease current fuel value, while in the chainsaw state
pub fn drain_fuel(
    mut player_info: ResMut<PlayerInfo>,
    mut player_transition_to_regular_form_event_writer: EventWriter<
        PlayerTransitionToRegularFormEvent,
    >,
    time: Res<Time>,
) {
    player_info.current_fuel -= CHAINSAW_FUEL_DRAIN_SPEED * time.delta_seconds();

    if player_info.current_fuel < 1.0 {
        player_transition_to_regular_form_event_writer.send(PlayerTransitionToRegularFormEvent {});
    }
}

// Player will slowly gain fuel over time in regular state
pub fn gain_fuel_over_time(mut player_info: ResMut<PlayerInfo>, time: Res<Time>) {
    let fuel_gain_amount =
        PASSIVE_PLAYER_FUEL_GAIN_AMOUNT * PASSIVE_PLAYER_FUEL_GAIN_SPEED * time.delta_seconds();
    change_player_fuel(&mut player_info, fuel_gain_amount);
}

pub fn check_player_pickup_collision(
    mut commands: Commands,
    mut player_info: ResMut<PlayerInfo>,
    player_query: Query<(&Transform, &Player)>,
    fuel_query: Query<(Entity, &Transform, &Pickup), With<FuelPickup>>,
    health_query: Query<(Entity, &Transform, &Pickup), With<HealthPickup>>,
) {
    if let Ok((player_transform, player_struct)) = player_query.get_single() {
        for (fuel_entity, fuel_transform, fuel_struct) in fuel_query.iter() {
            // If collided with fuel
            if let Some(_) = collide(
                player_transform.translation,
                player_struct.collider.size,
                fuel_transform.translation,
                fuel_struct.collider.size,
            ) {
                player_info.current_fuel =
                    if PLAYER_FUEL_CAPACITY < player_info.current_fuel + FUEL_PICKUP_RESTORE {
                        PLAYER_FUEL_CAPACITY
                    } else {
                        player_info.current_fuel + FUEL_PICKUP_RESTORE
                    };
                commands.entity(fuel_entity).despawn();
            }
        }

        for (health_entity, health_transform, health_struct) in health_query.iter() {
            // If collided with heart
            if let Some(_) = collide(
                player_transform.translation,
                player_struct.collider.size,
                health_transform.translation,
                health_struct.collider.size,
            ) {
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

// While colliding with enemy, chainsaw will gradually overheat
// When the player will turn completely orange, the fire wave will be released
// Setting all the enemies on fire
pub fn check_player_enemy_collision(
    mut player_take_damage_event_writer: EventWriter<PlayerTakeDamageEvent>,
    mut enemy_take_damage_event_writer: EventWriter<EnemyTakeDamageEvent>,
    mut enemies_query: Query<(&Transform, Entity, &Enemy)>,
    player_query: Query<(&Transform, &Player)>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok((player_transform, player_struct)) = player_query.get_single() {
        for (enemy_transform, enemy_entity, enemy_struct) in enemies_query.iter_mut() {
            // If collided with enemy
            if let Some(_) = collide(
                player_transform.translation,
                player_struct.collider.size,
                enemy_transform.translation,
                enemy_struct.collider.size,
            ) {
                // Check in which state player is
                match player_state.0 {
                    PlayerState::REGULAR => {
                        player_take_damage_event_writer.send(PlayerTakeDamageEvent {});
                        return;
                    }
                    // If the player already took damage
                    PlayerState::DAMAGED => {
                        return;
                    }
                    // Send the event, when enemy takes damage
                    PlayerState::CHAINSAW => {
                        enemy_take_damage_event_writer.send(EnemyTakeDamageEvent { enemy_entity });
                    }
                };
            }
        }
    }
}

pub fn check_player_projectile_collision(
    mut player_take_damage_event_writer: EventWriter<PlayerTakeDamageEvent>,
    mut projectiles_query: Query<(&Transform, &mut Projectile)>,
    player_query: Query<(&Transform, &Player)>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok((player_transform, player_struct)) = player_query.get_single() {
        for (projectile_transform, mut projectile_struct) in projectiles_query.iter_mut() {
            // If collided with projectile
            if let Some(_) = collide(
                player_transform.translation,
                player_struct.collider.size,
                projectile_transform.translation,
                projectile_struct.collider.size,
            ) {
                // Check in which state player is
                match player_state.0 {
                    PlayerState::REGULAR => {
                        player_take_damage_event_writer.send(PlayerTakeDamageEvent {});
                        return;
                    }
                    // If the player already took damage
                    PlayerState::DAMAGED => {
                        continue;
                    }
                    // Send projectiles backwards
                    PlayerState::CHAINSAW => {
                        projectile_struct.direction *= -1.;
                        continue;
                    }
                };
            }
        }
    }
}

// Executes when enemy takes damage
pub fn manage_chainsaw_overheat(
    mut enemy_take_damage_event_reader: EventReader<EnemyTakeDamageEvent>,
    mut fire_wave_event_writer: EventWriter<ChainsawFireWave>,
    mut player_info: ResMut<PlayerInfo>,
    mut player_query: Query<&mut Sprite, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut player_sprite) = player_query.get_single_mut() {
        for _ in enemy_take_damage_event_reader.iter() {
            player_info.chainsaw_heat += CHAINSAW_HEAT_SPEED * time.delta_seconds();
            // println!("Chainsaw heat: {}", player_info.chainsaw_heat);

            // Gradually turn orange
            player_sprite.color = Color::rgb(
                player_sprite.color.r(),
                player_sprite.color.g() - 0.01 * CHAINSAW_HEAT_SPEED * time.delta_seconds(),
                player_sprite.color.b(),
            );

            if player_info.chainsaw_heat >= CHAINSAW_HEAT_LIMIT {
                fire_wave_event_writer.send(ChainsawFireWave {});
                player_info.chainsaw_heat = 0.0;
                player_sprite.color = Color::WHITE;
                return;
            }
        }
    }
}

pub fn move_player(
    mut player_query: Query<(&mut Transform, &Player)>,
    game_info: Res<GameInfo>,
    time: Res<Time>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let cursor_position = game_info.cursor_position;
        let destination = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
        let direction = (destination - player_transform.translation).normalize();

        if player_transform.translation.distance(destination) > 10.0 {
            player_transform.translation += direction * player.current_speed * time.delta_seconds();
        }
    }
}

pub fn update_player_progress(mut game_info: ResMut<GameInfo>, time: Res<Time>) {
    if game_info.player_progress >= MAX_DEPTH {
        return;
    }

    game_info.player_progress += PLAYER_FALLING_SPEED * time.delta_seconds();
    println!("{}", game_info.player_progress);
}

pub fn limit_player_movement(
    mut player_query: Query<(&mut Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player_transform, player_struct)) = player_query.get_single_mut() {
        let primary_window = window_query.get_single().unwrap();
        let x_max = primary_window.width() - player_struct.collider.size.x;
        let x_min = 0.0 + player_struct.collider.size.x;
        let y_max = primary_window.height() - player_struct.collider.size.y;
        let y_min = 0.0 + player_struct.collider.size.y;

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

// Runs only when the player is in the regular state.
// When player has no health, send game over event.
// Otherwise, decrement health and make player invulnerable for 1 second.
pub fn handle_player_take_damage_event(
    mut player_take_damage_event_reader: EventReader<PlayerTakeDamageEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_info: ResMut<PlayerInfo>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for _ in player_take_damage_event_reader.iter() {
        if player_info.current_hp <= 1 {
            game_over_event_writer.send(GameOverEvent {});
            player_info.current_hp = 0;
        } else {
            next_player_state.set(PlayerState::DAMAGED);
            player_info.current_hp -= 1;
        }
        return;
    }
}

// Player sprite becomes a bit transparent, and the corresponding sound plays
pub fn player_take_damage_invulnerability(
    mut player_query: Query<&mut Sprite, With<Player>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut damage_invulnerability_timer: ResMut<PlayerDamageInvulnerabilityTimer>,
) {
    if let Ok(mut player_sprite) = player_query.get_single_mut() {
        player_sprite.color.set_a(0.5);

        if damage_invulnerability_timer.timer.just_finished() {
            next_player_state.set(PlayerState::REGULAR);
            player_sprite.color.set_a(1.0);
            damage_invulnerability_timer.timer.reset();
        }
    }
}

pub fn change_player_fuel(player_info: &mut PlayerInfo, amount: f32) {
    player_info.current_fuel = if player_info.current_fuel + amount < 0.0 {
        0.0
    } else if player_info.current_fuel + amount > PLAYER_FUEL_CAPACITY {
        PLAYER_FUEL_CAPACITY
    } else {
        player_info.current_fuel + amount
    }
}

pub fn tick_damage_invulnerability_timer(
    mut damage_invulnerability_timer: ResMut<PlayerDamageInvulnerabilityTimer>,
    time: Res<Time>,
) {
    damage_invulnerability_timer.timer.tick(time.delta());
}
