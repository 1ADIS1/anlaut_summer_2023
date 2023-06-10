use super::components::Player;
use super::resources::PlayerDamageInvulnerabilityTimer;
use super::{PlayerInfo, PlayerState};
use super::{
    PLAYER_CHAINSAW_SPEED, PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH, PLAYER_REGULAR_SPEED,
    PLAYER_SPRITE_SIZE,
};
use crate::game::components::{FuelPickup, HealthPickup};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SPRITE_SIZE;
use crate::game::events::{EnemyTakeDamageEvent, GameOverEvent, PlayerTakeDamageEvent};
use crate::game::GameInfo;
use crate::game::{
    CHAINSAW_FUEL_DRAIN_SPEED, FUEL_PICKUP_RESTORE, HEALTH_PICKUP_RESTORE, PICKUP_SPRITE_SIZE,
};

use bevy::prelude::*;
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
            texture: asset_server.load("sprites/player.png"),
            ..default()
        },
        Player {
            current_speed: PLAYER_REGULAR_SPEED,
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
            *player_texture = asset_server.load("sprites/chainsaw_form.png");
            player.current_speed = PLAYER_CHAINSAW_SPEED;

            println!("Entered chainsaw mode!");
        }
    }
}

// TODO: plays the animation, sound and shader.
pub fn transition_to_player_regular_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<(&mut Handle<Image>, &mut Player)>,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((mut player_texture, mut player)) = player_query.get_single_mut() {
        if player_info.current_fuel < 1.0 {
            next_player_state.set(PlayerState::REGULAR);
            *player_texture = asset_server.load("sprites/player.png");
            player.current_speed = PLAYER_REGULAR_SPEED;
            println!("Returned back to regular form!");
        }
    }
}

// Decrease current fuel value, while in the chainsaw state
pub fn drain_fuel(mut player_info: ResMut<PlayerInfo>, time: Res<Time>) {
    player_info.current_fuel -= CHAINSAW_FUEL_DRAIN_SPEED * time.delta_seconds();
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

pub fn check_player_enemy_collision(
    mut player_take_damage_event_writer: EventWriter<PlayerTakeDamageEvent>,
    mut enemy_take_damage_event_writer: EventWriter<EnemyTakeDamageEvent>,
    mut enemies_query: Query<(&Transform, Entity), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_radius = PLAYER_SPRITE_SIZE / 2.0;
        let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;

        for (enemy_transform, enemy_entity) in enemies_query.iter_mut() {
            // Player has collided with enemy
            if player_transform
                .translation
                .distance(enemy_transform.translation)
                < player_radius + enemy_radius
            {
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

// TODO: fix 'glitchy' movement
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

pub fn tick_damage_invulnerability_timer(
    mut damage_invulnerability_timer: ResMut<PlayerDamageInvulnerabilityTimer>,
    time: Res<Time>,
) {
    damage_invulnerability_timer.timer.tick(time.delta());
}
