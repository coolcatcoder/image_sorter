#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::cast_precision_loss)]

use bevy::{prelude::*, window::ExitCondition};

mod images;
mod input;
mod macros;
mod ui;
mod windows;

// plugin! {
//     mod windows;

//     //plugin DefaultPlugins;

//     //system create_secondary_window => Startup;
// }

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            }),
            windows::plugin,
            ui::plugin,
            images::plugin,
            input::plugin,
        ))
        .run()
}
