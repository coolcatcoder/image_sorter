use std::fmt::Debug;

use bevy::{
    color::palettes::css::BLACK,
    prelude::*,
    ui_widgets::popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide},
};
use unkindness::prelude::Else;

use crate::{
    bundle_effect,
    ui::{
        InnerUiSetup,
        fonts::{Font, Fonts},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn.in_set(InnerUiSetup))
        .add_observer(popover_closer);
}

#[derive(Component)]
pub struct Root;
pub fn spawn(mut commands: Commands, fonts: Res<Fonts>) {
    let buttons = ["File", "Edit", "Render", "Window", "Help"].map(|text| {
        let text = commands.spawn(fonts.workflow_bar(text)).id();
        commands
            .spawn((
                Node {
                    //width: px(30),
                    height: percent(85),
                    ..default()
                },
                BackgroundColor(BLACK.lighter(0.03).into()),
            ))
            .add_child(text)
            .id()
    });

    let mut bar = commands.spawn((
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
    ));

    bar.add_children(&buttons);

    bar.with_children(|commands| {
        commands.spawn(ButtonNew("Last.")).observe(
            // Consider a custom ButtonPressed event to automatically opt-out of bubbling. It might
            // even be nice for disabling buttons from being pressed when there is a popup.
            |on: On<Pointer<Click>>, mut commands: Commands| {
                if on.entity != on.original_event_target() {
                    return;
                }

                info!("Clicked new.");
                let dropdown = commands.spawn(Dropdown(on.entity)).id();
                let _button = commands
                    .spawn((ChildOf(dropdown), ButtonNew("Sub-button.")))
                    .observe(|on: On<Pointer<Click>>, mut commands: Commands| {
                        if on.entity != on.original_event_target() {
                            return;
                        }

                        info!("Sub-button pressed.");
                        let _dropdown = commands.spawn(Dropdown(on.entity)).id();
                    })
                    .id();
            },
        );
    });
}

struct Dropdown(Entity);
bundle_effect!(impl Effect for Dropdown);
impl Dropdown {
    fn effect(In((entity, parent)): In<(Entity, Self)>, mut commands: Commands) {
        commands.entity(entity).insert((
            ChildOf(parent.0),
            OverrideClip,
            Popover {
                positions: [PopoverPlacement {
                    side: PopoverSide::Bottom,
                    align: PopoverAlign::Start,
                    gap: 0.,
                }]
                .into(),
                window_margin: 0.,
            },
            BackgroundColor(BLACK.into()),
            Node {
                width: px(50),
                height: px(100),
                ..default()
            },
        ));
    }
}

struct ButtonNew(&'static str);
bundle_effect!(impl Effect for ButtonNew);
impl ButtonNew {
    fn effect(In((entity, button)): In<(Entity, Self)>, mut commands: Commands) {
        fn observer<E: Debug + Clone + Reflect>(
            colour: impl Into<Color>,
        ) -> impl Fn(On<Pointer<E>>, Query<&mut BackgroundColor>) {
            let colour = colour.into();
            move |on: On<Pointer<E>>, mut background_colour: Query<&mut BackgroundColor>| {
                background_colour.get_mut(on.entity).else_error()?.0 = colour;
            }
        }

        let on_colour = BLACK.lighter(0.03);
        let off_colour = BLACK;

        commands
            .entity(entity)
            .insert((
                Node {
                    height: percent(85),
                    ..default()
                },
                BackgroundColor::default(),
            ))
            .observe(observer::<Enter>(on_colour))
            .observe(observer::<Leave>(off_colour))
            .with_child((Text::new(button.0), Font::WorkflowBar));
    }
}

fn popover_closer(
    on: On<Pointer<Click>>,
    popover: Query<Entity, With<Popover>>,
    child_of: Query<&ChildOf>,
    ignore: Query<AnyOf<(&Window,)>>,
    mut commands: Commands,
) {
    // Ignore window clicks and propagated clicks.
    if on.original_event_target() != on.entity || ignore.contains(on.entity) {
        return;
    }

    // If the popover isn't an ancestor of the clicked entity, then close it.
    'popover: for popover in popover {
        for ancestor in [on.entity]
            .into_iter()
            .chain(child_of.iter_ancestors(on.entity))
        {
            if popover == ancestor {
                continue 'popover;
            }
        }

        info!("Closed");
        commands.entity(popover).try_despawn();
    }
}
