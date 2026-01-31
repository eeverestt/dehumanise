use bevy::{math::U8Vec2, prelude::*};

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

#[derive(Resource)]
pub struct Sensitivity(pub U8Vec2);

impl Default for Sensitivity {
    fn default() -> Self {
        Self(U8Vec2::splat(50))
    }
}
