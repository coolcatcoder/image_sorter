use bevy::{color::palettes::css::BLACK, prelude::*};

pub fn plugin(_: &mut App) {}

pub fn bundle() -> impl Bundle {
    (
        BackgroundColor(BLACK.into()),
        Node {
            width: percent(100),
            height: px(26),
            ..default()
        },
    )
}
