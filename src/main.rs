mod game;
mod ui;

use bevy::prelude::*;
use game::GamePlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(UIPlugin)
        .run();
}
