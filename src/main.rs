mod game;
mod particle_system;
mod ui;

use game::{GamePlugin, GameState};
use particle_system::ParticleSystemPlugin;
use ui::UIPlugin;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Anlaut Jam".into(),
                        resolution: (260. * 2.0, 320. * 2.0).into(),
                        present_mode: PresentMode::AutoVsync,
                        // mode: WindowMode::BorderlessFullscreen,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<GameState>()
        .add_plugin(ParticleSystemPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .run();
}
