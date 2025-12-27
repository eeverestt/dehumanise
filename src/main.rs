use crate::camera::plugin::CameraPlugin;
use crate::debug::plugin::DebugPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::settings::plugin::SettingsPlugin;
use crate::ui::plugin::UiPlugin;

use bevy::prelude::*;

pub mod debug {
    pub mod plugin;
}

pub mod player {
    pub mod components;
    pub mod plugin;
    pub mod systems;
}

pub mod settings {
    pub mod plugin;
    pub mod resources;
    pub mod systems;
}

pub mod ui {
    pub mod plugin;
    pub mod states;
    pub mod systems;
}

pub mod camera {
    pub mod components;
    pub mod plugin;
    pub mod systems;
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            DebugPlugin,
            PlayerPlugin,
            SettingsPlugin,
            UiPlugin,
            CameraPlugin,
        ))
        .run();
}
