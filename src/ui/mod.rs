mod components;
mod styles;

use crate::game::GameState;
use components::*;
use styles::*;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_game_ui.in_schedule(OnEnter(GameState::RUNNING)));
    }
}

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let fuel_bar_entity = build_fuel_bar(&mut commands, &asset_server);
}

pub fn build_fuel_bar(mut commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands.spawn((NodeBundle { ..default() }, GameUI {})).id()
}
