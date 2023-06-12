use super::components::*;
use super::styles::*;
use super::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};
use crate::game::events::CounterAttackFailed;
use crate::game::events::CounterAttackSucceeded;
use crate::game::events::EnemyCounterAttackEvent;
use crate::game::player::{PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH};
use crate::game::CounterAttackTimer;
use crate::game::COUNTER_ATTACK_DURATION;
use crate::game::COUNTER_ATTACK_MICE_NUMBER;
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

pub fn update_counter_attack_timer_text(
    mut counter_attack_timer_text_query: Query<&mut Text, With<CounterAttackTimerText>>,
    counter_attack_timer: Res<CounterAttackTimer>,
) {
    if counter_attack_timer.is_changed() {
        if let Ok(mut counter_attack_timer_text) = counter_attack_timer_text_query.get_single_mut()
        {
            counter_attack_timer_text.sections[0].value = format!(
                "{:.4}",
                counter_attack_timer
                    .timer
                    .remaining_secs()
                    // .floor()
                    .to_string()
            );
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

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_ui(&mut commands, &asset_server);
}

pub fn spawn_counter_attack_ui(
    mut commands: Commands,
    mut enemy_counter_attack_event_reader: EventReader<EnemyCounterAttackEvent>,
    asset_server: Res<AssetServer>,
) {
    for counter_attack_event in enemy_counter_attack_event_reader.iter() {
        build_counter_attack_ui(
            &mut commands,
            &asset_server,
            &counter_attack_event.keys_to_press,
            COUNTER_ATTACK_DURATION,
        );
        return;
    }
}

pub fn despawn_counter_attack_ui(
    mut commands: Commands,
    mut counter_attack_event_failed_reader: EventReader<CounterAttackFailed>,
    mut counter_attack_event_succeeded_reader: EventReader<CounterAttackSucceeded>,
    counter_attack_ui_query: Query<Entity, With<CounterAttackUI>>,
) {
    if let Ok(counter_attack_ui_entity) = counter_attack_ui_query.get_single() {
        for _ in counter_attack_event_failed_reader.iter() {
            commands
                .entity(counter_attack_ui_entity)
                .despawn_recursive();
            return;
        }

        for _ in counter_attack_event_succeeded_reader.iter() {
            commands
                .entity(counter_attack_ui_entity)
                .despawn_recursive();
            return;
        }
    }
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

pub fn build_counter_attack_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    keys_to_press: &Vec<MouseButton>,
    counter_attack_time: f32,
) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: GAME_HUD_STYLE,
                ..default()
            },
            CounterAttackUI {},
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    // Center UI elements
                    NodeBundle {
                        style: CENTER_STYLE,
                        ..default()
                    },
                )
                .with_children(|parent| {
                    // ===== Counter Attack BG Image =====
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(200.0), Val::Px(106.0)),
                                margin: UiRect::new(
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                    Val::Px(32.0),
                                    Val::Px(8.0),
                                ),
                                ..default()
                            },
                            image: asset_server.load("sprites/blood_bg.png").into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // ===== LMBs and RMBs =====
                            for i in 0..COUNTER_ATTACK_MICE_NUMBER {
                                parent.spawn(ImageBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                        ),
                                        ..default()
                                    },
                                    image: if keys_to_press[i] == MouseButton::Left {
                                        asset_server.load("sprites/LMB.png").into()
                                    } else {
                                        asset_server.load("sprites/RMB.png").into()
                                    },
                                    ..default()
                                });
                            }
                            // ==== Timer ====
                            parent.spawn(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                                    margin: UiRect::new(
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                        Val::Px(8.0),
                                    ),
                                    ..default()
                                },
                                image: asset_server.load("sprites/clock.png").into(),
                                ..default()
                            });

                            // ==== Timer text ====
                            parent.spawn((
                                TextBundle {
                                    style: Style {
                                        margin: UiRect::new(
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                            Val::Px(8.0),
                                        ),
                                        ..default()
                                    },
                                    text: Text {
                                        sections: vec![TextSection::new(
                                            counter_attack_time.to_string(),
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/origami_mommy_regular.ttf"),
                                                font_size: 32.0,
                                                color: Color::WHITE,
                                            },
                                        )],
                                        alignment: TextAlignment::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                CounterAttackTimerText {},
                            ));
                        });
                });
        })
        .id()
}

// pub fn despawn_counter_attack_hud(mut commands: Commands) {}

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

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // === Play Button ===
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..default()
                    },
                    PlayButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                get_button_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}
