use super::components::*;
use super::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::game::{player::resources::PlayerInfo, GameState};

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

pub fn interact_with_play_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
