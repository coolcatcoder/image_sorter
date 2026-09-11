use std::marker::PhantomData;

use bevy::{
    color::palettes::css::{BLACK, RED},
    ecs::system::SystemId,
    prelude::*,
    ui_widgets::popover::{Popover, PopoverAlign, PopoverPlacement, PopoverSide},
};

use crate::{
    bundle_effect,
    ui::{
        InnerUiSetup,
        fonts::{Font, Fonts},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn.in_set(InnerUiSetup))
        .add_systems(Update, button_interaction)
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

    let workflow = commands
        .spawn((Text::new("Workflow"), Font::WorkflowBar))
        .id();
    let workflow = commands.spawn(workflow_bar_item()).add_child(workflow).id();

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

    bar.add_children(&buttons)
        .add_child(workflow)
        .with_child(Button(
            "Tester!",
            |In(entity): In<Entity>, mut commands: Commands| {
                info!("Pressed!");
                commands.spawn((
                    ChildOf(entity),
                    OverrideClip,
                    Popover {
                        positions: [PopoverPlacement {
                            side: PopoverSide::Bottom,
                            align: PopoverAlign::Start,
                            gap: 1.,
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
            },
        ));

    bar.with_child(Button(
        "Incredible!",
        |In(entity): In<Entity>, mut commands: Commands| {
            commands.spawn(Dropdown(entity));
        },
    ));
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

struct Button<F: IntoSystem<In<Entity>, (), M> + 'static, M>
where
    Self: Send + Sync + 'static,
{
    text: &'static str,
    system: F,
    phantom_data: PhantomData<M>,
}
#[allow(nonstandard_style)]
fn Button<F: IntoSystem<In<Entity>, (), M> + 'static, M>(
    text: &'static str,
    system: F,
) -> Button<F, M>
where
    Button<F, M>: Send + Sync + 'static,
{
    Button {
        text,
        system,
        phantom_data: PhantomData,
    }
}
bundle_effect!(impl<F: IntoSystem<In<Entity>, (), M> + 'static, M> Effect for Button<F, M> where Self: Send + Sync + 'static);
impl<F: IntoSystem<In<Entity>, (), M> + 'static, M> Button<F, M>
where
    Self: Send + Sync + 'static,
{
    fn effect(In((entity, button)): In<(Entity, Self)>, mut commands: Commands) {
        let system_id = commands.register_system(button.system);
        commands
            .entity(entity)
            .insert((
                ButtonSystem(system_id),
                Interaction::None,
                Node {
                    height: percent(85),
                    ..default()
                },
                BackgroundColor::default(),
            ))
            .with_child((Text::new(button.text), Font::WorkflowBar));
    }
}

#[derive(Component)]
struct ButtonSystem(SystemId<In<Entity>, ()>);
fn button_interaction(
    interaction: Query<
        (Entity, &Interaction, &ButtonSystem, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut commands: Commands,
) {
    for (entity, interaction, button_system, mut background_colour) in interaction {
        match *interaction {
            Interaction::None => background_colour.0 = BLACK.into(),
            Interaction::Hovered => background_colour.0 = BLACK.lighter(0.03).into(),
            Interaction::Pressed => commands.run_system_with(button_system.0, entity),
        }
    }
}

fn popover_closer(
    on: On<Pointer<Click>>,
    popover: Query<(Entity, Option<&ChildOf>), With<Popover>>,
    child_of: Query<&ChildOf>,
    ignore: Query<AnyOf<(&Window,)>>,
    mut commands: Commands,
) {
    // Ignore window clicks and propagated clicks.
    if on.original_event_target() != on.entity || ignore.contains(on.entity) {
        return;
    }

    // If the popover isn't an ancestor of the clicked entity, then close it.
    'popover: for (popover, popover_parent) in popover {
        // If the entity clicked is the popover's parent then it is probably the button that opened
        // it. Closing it as soon as it opens is silly, and this should avoid that.
        if let Some(popover_parent) = popover_parent
            && popover_parent.0 == on.entity
        {
            continue 'popover;
        }

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

fn workflow_bar_item() -> impl Bundle {
    (
        Node {
            height: percent(85),
            ..default()
        },
        BackgroundColor(BLACK.lighter(0.03).into()),
    )
}
