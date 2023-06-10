use super::components::ParallaxBackground;
use super::PARALLAX_SPEED;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_parallax_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    let bg_size = Vec2::new(320.0 * 3.0, 180.0 * 3.0);

    // Top background. Spawn it at the center of the screen.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                -1.0,
            ),
            texture: asset_server.load("sprites/bg1.png"),
            sprite: Sprite {
                custom_size: Some(bg_size),
                ..default()
            },
            ..default()
        },
        ParallaxBackground { size: bg_size },
    ));

    // Bottom background. Spawn it outside of the screen.
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                -(primary_window.height() / 2.0),
                -1.0,
            ),
            texture: asset_server.load("sprites/bg1.png"),
            sprite: Sprite {
                custom_size: Some(bg_size),
                ..default()
            },
            ..default()
        },
        ParallaxBackground { size: bg_size },
    ));
}

// Moves background up
// When goes out of the bounds -> move it at the bottom of the screen and move it for one additional frame more
// TODO: fix overheads
pub fn move_parallax_background(
    mut parallax_background_query: Query<(&mut Transform, &ParallaxBackground)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let primary_window = window_query.get_single().unwrap();
    let parallax_direction = Vec3::new(0.0, 1.0, 0.0).normalize();

    for (mut parallax_bg_transform, _) in parallax_background_query.iter_mut() {
        parallax_bg_transform.translation +=
            parallax_direction * PARALLAX_SPEED * time.delta_seconds();

        if parallax_bg_transform.translation.y - primary_window.height() / 2.0
            > primary_window.height()
        {
            parallax_bg_transform.translation.y =
                -(primary_window.height() / 2.0) + PARALLAX_SPEED * time.delta_seconds();
            continue;
        }
    }
}
