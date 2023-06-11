mod components;
mod styles;
mod systems;

use crate::game::player::{PLAYER_FUEL_CAPACITY, PLAYER_MAX_HEALTH};
use crate::game::{GameState, COUNTER_ATTACK_MICE_NUMBER};
use components::*;
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
            spawn_counter_attack_ui.in_schedule(OnEnter(GameState::CounterAttack)),
            despawn_counter_attack_ui.in_schedule(OnExit(GameState::CounterAttack)),
        ));
    }
}

pub fn spawn_game_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_game_ui(&mut commands, &asset_server);
}

pub fn spawn_counter_attack_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_counter_attack_ui(&mut commands, &asset_server);
}

pub fn despawn_counter_attack_ui(
    mut commands: Commands,
    counter_attack_ui_query: Query<Entity, With<CounterAttackUI>>,
) {
    if let Ok(counter_attack_ui_entity) = counter_attack_ui_query.get_single() {
        commands
            .entity(counter_attack_ui_entity)
            .despawn_recursive();
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

pub fn build_counter_attack_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
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
                                size: Size::new(Val::Px(128.0), Val::Px(53.0)),
                                margin: UiRect::new(
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                    Val::Px(8.0),
                                ),
                                ..default()
                            },
                            image: asset_server.load("sprites/blood_bg.png").into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // ===== LMBs and RMBs =====
                            for _ in 0..COUNTER_ATTACK_MICE_NUMBER {
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
                                    image: asset_server.load("sprites/LMB.png").into(),
                                    ..default()
                                });
                            }
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
