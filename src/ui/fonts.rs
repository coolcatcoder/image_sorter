use bevy::{
    prelude::*,
    text::{FontVariationTag, FontVariations},
};

use crate::bundle_effect;

pub fn plugin(app: &mut App) {
    let fonts = Fonts::new(app.world().resource::<AssetServer>());
    app.insert_resource(fonts);
}

#[derive(Debug)]
pub enum Font {
    Body,
    WorkflowBar,
}
bundle_effect!(impl Effect for Font);
impl Font {
    fn effect(
        In((entity, font)): In<(Entity, Self)>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let load = |path| FontSource::Handle(asset_server.load(path));
        let font = match font {
            Self::Body => TextFont {
                font: load("garamond.ttf"),
                font_size: FontSize::Px(50.),
                weight: FontWeight::NORMAL,
                ..default()
            },
            Self::WorkflowBar => TextFont {
                font: load("google.ttf"),
                font_size: FontSize::Px(11.),
                weight: FontWeight(10),
                font_variations: FontVariations::builder()
                    .set(FontVariationTag::OPTICAL_SIZE, 1.)
                    .build(),
                ..default()
            },
        };
        commands.entity(entity).insert((
            font,
            Pickable {
                should_block_lower: false,
                is_hoverable: false,
            },
        ));
    }
}

#[derive(Resource)]
pub struct Fonts {
    body: TextFont,
    workflow_bar: TextFont,
}

impl Fonts {
    fn new(asset_server: &AssetServer) -> Self {
        Self {
            body: TextFont {
                font: FontSource::Handle(asset_server.load("garamond.ttf")),
                font_size: FontSize::Px(50.),
                weight: FontWeight::NORMAL,
                ..default()
            },
            workflow_bar: TextFont {
                font: FontSource::Handle(asset_server.load("google.ttf")),
                font_size: FontSize::Px(11.),
                weight: FontWeight(10),
                font_variations: FontVariations::builder()
                    .set(FontVariationTag::OPTICAL_SIZE, 1.)
                    .build(),
                ..default()
            },
        }
    }
    pub fn body(&self, text: impl Into<String>) -> impl Bundle {
        (self.body.clone(), Text::new(text))
    }
    pub fn workflow_bar(&self, text: impl Into<String>) -> impl Bundle {
        (self.workflow_bar.clone(), Text::new(text))
    }
}
