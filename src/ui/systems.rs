use super::components::*;
use crate::game::player::resources::PlayerInfo;

use bevy::prelude::*;

// Updates all the game ui, if the player_info got changed
pub fn update_ui_text(
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<FuelText>)>,
    mut fuel_text_query: Query<&mut Text, (With<FuelText>, Without<HealthText>)>,
    // mut fuel_text_query: Query<&mut Text, With<FuelText>>,
    player_info: Res<PlayerInfo>,
) {
    if player_info.is_changed() {
        for mut health_text in health_text_query.iter_mut() {
            health_text.sections[0].value = format!("{}", player_info.current_hp.to_string());
        }

        for mut fuel_text in fuel_text_query.iter_mut() {
            fuel_text.sections[0].value =
                format!("{}", player_info.current_fuel.floor().to_string());
        }
    }
}
