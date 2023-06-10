mod game;
mod ui;

use bevy::{prelude::*, window::PresentMode};
use game::GamePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Anlaut Jam".into(),
                resolution: (960., 540.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                resizable: false,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .run();
}
