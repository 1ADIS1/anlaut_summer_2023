use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_REGULAR_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameInfo>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(get_cursor_world_coordinates)
        .add_system(move_player)
        .run();
}

// Stores every useful information for our game
#[derive(Resource, Default)]
pub struct GameInfo {
    cursor_position: Vec2,
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            ),
            texture: asset_server.load("ball_blue_large.png"),
            ..default()
        },
        Player {
            speed: PLAYER_REGULAR_SPEED,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            ),
            ..default()
        },
        MainCamera {},
    ));
}

pub fn get_cursor_world_coordinates(
    mut game_info: ResMut<GameInfo>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let primary_window = window_query.get_single().unwrap();

    if let Some(cursor_world_position) = primary_window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        game_info.cursor_position = cursor_world_position;
    }
}

pub fn move_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    game_info: Res<GameInfo>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        let cursor_position = game_info.cursor_position;
        player.translation = Vec3::new(cursor_position.x, cursor_position.y, 0.0);
    }
}
