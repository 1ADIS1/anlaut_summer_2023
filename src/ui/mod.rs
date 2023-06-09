mod components;
mod styles;
mod systems;

use crate::game::player::PLAYER_MAX_HEALTH;
use crate::game::GameState;
use components::*;
use styles::*;
use systems::*;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((
            spawn_game_ui.in_schedule(OnEnter(GameState::RUNNING)),
            update_ui_text.run_if(in_state(GameState::RUNNING)),
        ));
    }
}

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_ui_entity = build_game_ui(&mut commands, &asset_server);
}

pub fn build_game_ui(mut commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center, // TODO: sanity check
                    align_items: AlignItems::Center,         // TODO: sanity check
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    gap: Size::new(Val::Px(8.0), Val::Px(8.0)), // TODO: sanity check
                    ..default()
                },
                ..default()
            },
            GameUI {},
        ))
        .with_children(|parent| {
            // === Health bar === (LHS, top)
            parent
                .spawn(
                    // Health bar image
                    ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(129.0), Val::Px(63.0)),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::new(
                                Val::Px(32.0),
                                Val::Px(0.0),
                                Val::Px(0.0),
                                Val::Px(0.0),
                            ),
                            ..default()
                        },
                        image: asset_server.load("sprites/health_bg.png").into(),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    parent.spawn(
                        // Health bar text
                        (
                            TextBundle {
                                text: Text {
                                    sections: vec![TextSection::new(
                                        PLAYER_MAX_HEALTH.to_string(),
                                        TextStyle {
                                            font: asset_server
                                                .load("fonts/origami_mommy_regular.ttf"),
                                            font_size: 64.0,
                                            color: Color::WHITE,
                                        },
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            },
                            HealthText {},
                        ),
                    );
                });

            // === Fuel bar === (LHS, bottom)

            // === Blood score === (RHS, bottom)
        })
        .id()
}
