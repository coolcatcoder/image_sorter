use bevy::{
    color::palettes::css::{BLUE, DARK_GREY},
    prelude::*,
};
use fonts::Fonts;

mod fonts;
mod slider;

mod workflow_bar;

use crate::{images::Images, windows::PrimaryCamera};

pub fn plugin(app: &mut App) {
    app.add_plugins((slider::plugin, workflow_bar::plugin))
        .add_systems(
            Startup,
            create_ui
                .after(crate::images::Setup)
                .after(crate::windows::Setup),
        );
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_camera: Single<Entity, With<PrimaryCamera>>,
    secondary_cameras: Query<Entity, (With<Camera>, Without<PrimaryCamera>)>,
    images: Res<Images>,
) {
    let fonts = Fonts::new(&asset_server);

    let workflow_bar = commands.spawn(workflow_bar::bundle()).id();

    commands
        .spawn((
            UiTargetCamera(*primary_camera),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .add_children(&[workflow_bar]);

    /*
    commands
        .spawn((
            UiTargetCamera(*primary_camera),
            Node {
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                BackgroundColor(BLUE.into()),
                Node {
                    height: percent(10),
                    width: percent(100),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
            ))
            .with_children(|root| {
                root.spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: percent(0.5),
                    ..default()
                })
                .with_children(|root| {
                    let label = root.spawn(fonts.body("Default text!")).id();

                    root.spawn(slider::horizontal_slider("Test"));
                });
            });

            root.spawn(Node {
                width: percent(100),
                height: percent(90),
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|root| {
                //let height = percent(100. / images.0.len() as f32);
                for _ in &images.0 {
                    root.spawn((
                        Node {
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            margin: percent(0.5).vertical(),
                            ..default()
                        },
                        BackgroundColor(DARK_GREY.into()),
                    ))
                    .with_children(|root| {
                        for image in &images.0 {
                            root.spawn((
                                Node {
                                    width: percent(20),
                                    margin: percent(0.5).all(),
                                    //width: Val::Auto,
                                    //height,
                                    ..default()
                                },
                                ImageNode {
                                    image: image.clone(),
                                    image_mode: NodeImageMode::Auto,
                                    ..default()
                                },
                            ));
                        }
                    });
                }
            });
        });
    */

    // TODO: Add sliders above images for changing width of images and all that. https://bevy.org/examples/ui-user-interface/vertical-slider/

    if let Some(secondary_camera) = secondary_cameras.iter().next() {
        commands.spawn((
            UiTargetCamera(secondary_camera),
            Node { ..default() },
            fonts.body("Secondary!"),
        ));
    }
}
