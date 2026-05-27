use bevy::{
    input_focus::tab_navigation::TabIndex,
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::{
        Slider, SliderDragState, SliderRange, SliderThumb, SliderValue, TrackClick, observe,
        slider_self_update,
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, (update_slider_visuals, update_value_labels));
}

const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ValueLabel(pub Entity);

pub fn horizontal_slider(display: &'static str) -> impl Bundle {
    (
        //ValueLabel(label),
        observe(slider_self_update),
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Stretch,
            column_gap: px(4),
            height: px(12),
            width: px(200),
            ..default()
        },
        Hovered::default(),
        Slider {
            track_click: TrackClick::Snap,
            ..default()
        },
        SliderValue(50.0),
        SliderRange::new(0.0, 100.0),
        TabIndex(0),
        Children::spawn((
            Spawn((
                Node {
                    height: px(6),
                    border_radius: BorderRadius::all(px(3)),
                    ..default()
                },
                BackgroundColor(SLIDER_TRACK),
            )),
            Spawn((
                Node {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    left: px(0),
                    right: px(12),
                    top: px(0),
                    bottom: px(0),
                    ..default()
                },
                children![(
                    SliderThumb,
                    Node {
                        display: Display::Flex,
                        width: px(12),
                        height: px(12),
                        position_type: PositionType::Absolute,
                        left: percent(0),
                        border_radius: BorderRadius::MAX,
                        ..default()
                    },
                    BackgroundColor(SLIDER_THUMB),
                )],
            )),
        )),
    )
}

fn update_slider_visuals(
    sliders: Query<
        (
            Entity,
            &SliderValue,
            &SliderRange,
            &Hovered,
            &SliderDragState,
        ),
        Or<(
            Changed<SliderValue>,
            Changed<Hovered>,
            Changed<SliderDragState>,
        )>,
    >,
    children: Query<&Children>,
    mut thumbs: Query<(&mut Node, &mut BackgroundColor, Has<SliderThumb>), Without<Slider>>,
) {
    for (slider_ent, value, range, hovered, drag_state) in sliders.iter() {
        for child in children.iter_descendants(slider_ent) {
            if let Ok((mut thumb_node, mut thumb_bg, is_thumb)) = thumbs.get_mut(child)
                && is_thumb
            {
                let position = range.thumb_position(value.0) * 100.0;
                thumb_node.left = percent(position);

                let is_active = hovered.0 | drag_state.dragging;
                thumb_bg.0 = if is_active {
                    SLIDER_THUMB.lighter(0.3)
                } else {
                    SLIDER_THUMB
                };
            }
        }
    }
}

fn update_value_labels(
    sliders: Query<(&SliderValue, &ValueLabel), Changed<SliderValue>>,
    mut texts: Query<&mut Text>,
) {
    for (value, label) in sliders.iter() {
        if let Ok(mut text) = texts.get_mut(label.0) {
            **text = format!("{:.0}", value.0);
        }
    }
}

#[derive(Component, FromTemplate)]
struct TakesEntity(Entity);

fn tester() -> impl SceneList {
    bsn_list! [
        #EntityOne
        Transform,

        #EntityTwo
        TakesEntity(#Bad)
        Transform {
            scale: Vec3::default(),
        }
    ]
}

struct TheStruct;
#[macro_export]
macro_rules! TheStruct {
    () => {};
}
pub use crate::TheStruct;

mod blah {
    use super::TheStruct;

    type Bad = TheStruct;

    TheStruct!();
}

macro_rules! better {
    (
        $(
            $($entity_name:ident)? (
                $($expression:expr),*
                $(,)?
            )
        ),*
        $(,)?
    ) => {
        ($((
            $($expression),*
        )),*)
    };
}

fn other() {
    better!(
        name_here(Transform::default(), ()),
        wow_how_cool(Transform {
            scale: Vec3::default(),
            ..default()
        },)
    );
}
