use bevy::prelude::*;

pub struct PlayerTakeDamageEvent;

pub struct PlayerTransitionToRegularFormEvent;

pub struct GameOverEvent;

pub struct EnemyTakeDamageEvent {
    pub enemy_entity: Entity,
}

#[derive(Clone)]
pub struct EnemyCounterAttackEvent {
    pub enemy_entity: Entity,
    pub keys_to_press: Vec<MouseButton>,
}

impl Default for EnemyCounterAttackEvent {
    fn default() -> Self {
        EnemyCounterAttackEvent {
            enemy_entity: Entity::from_bits(0),
            keys_to_press: vec![],
        }
    }
}

pub struct CounterAttackFailed {
    pub enemy_entity: Entity,
}
