use bevy::prelude::*;

pub struct PlayerTakeDamageEvent;

pub struct PlayerTransitionToRegularFormEvent;

pub struct GameOverEvent;

pub struct EnemyTakeDamageEvent {
    pub enemy_entity: Entity,
}

pub struct ChainsawFireWave;
