use bevy::prelude::*;

use super::{PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH, PLAYER_TAKE_DAMAGE_INVULNERABILITY_PERIOD};

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

#[derive(Resource)]
pub struct PlayerDamageInvulnerabilityTimer {
    pub timer: Timer,
}

impl Default for PlayerDamageInvulnerabilityTimer {
    fn default() -> Self {
        PlayerDamageInvulnerabilityTimer {
            timer: Timer::from_seconds(PLAYER_TAKE_DAMAGE_INVULNERABILITY_PERIOD, TimerMode::Once),
        }
    }
}
