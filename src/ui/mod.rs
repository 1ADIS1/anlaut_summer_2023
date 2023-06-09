mod components;
mod styles;
mod systems;

use crate::game::player::{PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH};
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
    build_game_ui(&mut commands, &asset_server);
}

pub fn build_game_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: GAME_HUD_STYLE,
                ..default()
            },
            GameUI {},
        ))
        .with_children(|parent| {
            // LHS UI elements
            parent
                .spawn(NodeBundle {
                    style: LHS_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // === Health bar === (LHS, top)
                    parent
                        .spawn(
                            // Health bar image
                            ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(129.0), Val::Px(53.0)),
                                    margin: UiRect::new(
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
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
                                        style: Style {
                                            margin: UiRect::new(
                                                Val::Px(8.0),
                                                Val::Px(0.0),
                                                Val::Px(4.0),
                                                Val::Px(0.0),
                                            ),
                                            ..default()
                                        },
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                PLAYER_MAX_HEALTH.to_string(),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/origami_mommy_regular.ttf"),
                                                    font_size: 48.0,
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

                            parent.spawn(
                                // Health image
                                ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                        ),
                                        ..default()
                                    },
                                    image: asset_server.load("sprites/health.png").into(),
                                    ..default()
                                },
                            );
                        });

                    // === Fuel bar === (LHS, bottom)
                    parent
                        .spawn(
                            // Fuel bar image
                            ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(129.0), Val::Px(53.0)),
                                    margin: UiRect::new(
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                    ),
                                    ..default()
                                },
                                image: asset_server.load("sprites/fuel_bg.png").into(),
                                ..default()
                            },
                        )
                        .with_children(|parent| {
                            parent.spawn(
                                // Fuel bar text
                                (
                                    TextBundle {
                                        style: Style {
                                            margin: UiRect::new(
                                                Val::Px(8.0),
                                                Val::Px(0.0),
                                                Val::Px(4.0),
                                                Val::Px(0.0),
                                            ),
                                            ..default()
                                        },
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                PLAYER_FUEL_CAPACITY.to_string(),
                                                TextStyle {
                                                    font: asset_server
                                                        .load("fonts/origami_mommy_regular.ttf"),
                                                    font_size: 48.0,
                                                    color: Color::WHITE,
                                                },
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    FuelText {},
                                ),
                            );

                            parent.spawn(
                                // Fuel image
                                ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                        ),
                                        ..default()
                                    },
                                    image: asset_server.load("sprites/fuel.png").into(),
                                    ..default()
                                },
                            );
                        });
                });

            // RHS UI elements
            // === Blood score === (RHS, bottom)
        })
        .id()
}
