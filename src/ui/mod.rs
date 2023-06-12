mod components;
mod styles;
mod systems;

use super::game::GameState;
use styles::*;
use systems::*;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_main_menu.in_schedule(OnEnter(GameState::MainMenu)),
            despawn_main_menu.in_schedule(OnExit(GameState::MainMenu)),
            interact_with_play_button.run_if(in_state(GameState::MainMenu)),
            spawn_game_ui.in_schedule(OnExit(GameState::MainMenu)),
            update_ui_text.run_if(in_state(GameState::Running)),
            spawn_counter_attack_ui.run_if(in_state(GameState::CounterAttack)),
            update_counter_attack_timer_text.run_if(in_state(GameState::CounterAttack)),
            despawn_counter_attack_ui
                .run_if(in_state(GameState::CounterAttack))
                .before(spawn_counter_attack_ui),
        ));
    }
}
