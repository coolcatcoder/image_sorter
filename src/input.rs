use bevy::prelude::*;
use unkindness::input::{ActionTemplate, Input};

pub fn plugin(app: &mut App) {
    app.add_plugins(unkindness::input::plugin)
        .add_systems(Startup, register_bindings);
}

pub struct Up;
impl ActionTemplate for Up {
    type Template = bool;
}

pub struct Left;
impl ActionTemplate for Left {
    type Template = bool;
}

pub struct Right;
impl ActionTemplate for Right {
    type Template = bool;
}

pub struct Down;
impl ActionTemplate for Down {
    type Template = bool;
}

fn register_bindings(mut input: ResMut<Input>) {
    input.bind::<Up>(KeyCode::KeyK);
    input.bind::<Left>(KeyCode::KeyH);
    input.bind::<Right>(KeyCode::KeyL);
    input.bind::<Down>(KeyCode::KeyJ);
}
