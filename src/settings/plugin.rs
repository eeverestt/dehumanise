use crate::settings::resources::{Keymap, Sensitivity};
use bevy::prelude::*;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Sensitivity>();
        app.init_resource::<Keymap>();
    }
}
