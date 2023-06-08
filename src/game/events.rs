use bevy::prelude::*;

pub struct PlayerTakeDamageEvent;

pub struct GameOverEvent;

pub struct EnemyTakeDamageEvent {
    pub enemy_entity: Entity,
}
