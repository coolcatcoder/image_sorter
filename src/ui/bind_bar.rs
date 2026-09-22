use bevy::{color::palettes::css::BLACK, prelude::*};

use crate::ui::InnerUiSetup;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn.in_set(InnerUiSetup));
}

#[derive(Component)]
pub struct Root;
fn spawn(mut commands: Commands) {
    let _root = commands
        .spawn((
            Root,
            BackgroundColor(BLACK.into()),
            Node {
                width: percent(100),
                height: px(26),
                flex_direction: FlexDirection::Row,
                column_gap: px(5),
                align_items: AlignItems::Center,
                ..default()
            },
        ))
        .id();
}
