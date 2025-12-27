use bevy::prelude::*;

#[derive(Resource)]
pub struct Keymap {
    pub forward: KeyCode,
    pub left: KeyCode,
    pub back: KeyCode,
    pub right: KeyCode,
    pub shoot: MouseButton,
}

impl Default for Keymap {
    fn default() -> Self {
        Self {
            forward: KeyCode::KeyW,
            left: KeyCode::KeyA,
            back: KeyCode::KeyS,
            right: KeyCode::KeyD,
            shoot: MouseButton::Left,
        }
    }
}
