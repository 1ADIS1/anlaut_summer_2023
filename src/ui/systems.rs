use super::components::HealthText;
use crate::game::player::resources::PlayerInfo;

use bevy::prelude::*;

// Updates all the game ui, if the player_info got changed
pub fn update_ui_text(
    mut health_text_query: Query<&mut Text, With<HealthText>>,
    player_info: Res<PlayerInfo>,
) {
    if player_info.is_changed() {
        for mut health_text in health_text_query.iter_mut() {
            health_text.sections[0].value = format!("{}", player_info.current_hp.to_string());
        }
    }
}
