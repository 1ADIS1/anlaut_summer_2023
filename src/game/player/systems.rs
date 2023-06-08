use super::components::Player;
use super::resources::PlayerDamageInvulnerabilityTimer;
use super::{PlayerInfo, PlayerState};
use super::{PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH, PLAYER_SPRITE_SIZE};
use crate::game::enemy::components::Enemy;
use crate::game::enemy::ENEMY_SPRITE_SIZE;
use crate::game::events::{GameOverEvent, PlayerTakeDamageEvent};
use crate::game::{
    FuelPickup, GameInfo, HealthPickup, CHAINSAW_FUEL_DRAIN_SPEED, FUEL_PICKUP_RESTORE,
    HEALTH_PICKUP_RESTORE, PICKUP_SPRITE_SIZE,
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
        Player {},
    ));
}

// TODO: Plays the player chainsaw animation, sound and makes player invulnerable
// Transition to chainsaw state if the player:
// 1. In the regular form
// 2. Has maximum fuel
// 3. Clicks the LMB
pub fn transition_to_player_chainsaw_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    mouse_input: Res<Input<MouseButton>>,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut player_texture) = player_query.get_single_mut() {
        if player_info.current_fuel == PLAYER_FUEL_CAPACITY
            && mouse_input.just_pressed(MouseButton::Left)
        {
            next_player_state.set(PlayerState::CHAINSAW);
            *player_texture = asset_server.load("sprites/chainsaw_form.png");

            println!("Entered chainsaw mode!");
        }
    }
}

// TODO: plays the animation, sound and shader.
pub fn transition_to_player_regular_state(
    mut next_player_state: ResMut<NextState<PlayerState>>,
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    player_info: Res<PlayerInfo>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut player_texture) = player_query.get_single_mut() {
        if player_info.current_fuel < 1.0 {
            next_player_state.set(PlayerState::REGULAR);
            *player_texture = asset_server.load("sprites/player.png");
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
    player_query: Query<&Transform, With<Player>>,
    player_state: Res<State<PlayerState>>,
    enemies_query: Query<&Transform, With<Enemy>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_radius = PLAYER_SPRITE_SIZE / 2.0;
        let enemy_radius = ENEMY_SPRITE_SIZE / 2.0;

        for enemy_transform in enemies_query.iter() {
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
                    }
                    // If the player already took damage
                    PlayerState::DAMAGED => {
                        println!("Player is invulnerable!");
                    }
                    PlayerState::CHAINSAW => {
                        // TODO: enemy take damage
                    }
                };
            }
        }
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

// When player has no health, send game over event
// Otherwise, decrement health and make player invulnerable for 1 second.
pub fn handle_player_take_damage_event(
    mut player_take_damage_event_reader: EventReader<PlayerTakeDamageEvent>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    mut player_info: ResMut<PlayerInfo>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    for _ in player_take_damage_event_reader.iter() {
        if player_info.current_hp <= 0 {
            game_over_event_writer.send(GameOverEvent {});
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
    println!(
        "Invulnerability timer: {}",
        damage_invulnerability_timer.timer.elapsed_secs()
    );
}

pub fn player_info_updated(player_info: Res<PlayerInfo>) {
    if player_info.is_changed() {
        println!("Player info: {:?}", player_info);
    }
}
