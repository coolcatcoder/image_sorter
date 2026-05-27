use bevy::prelude::*;

#[derive(Resource)]
pub struct Fonts {
    body: TextFont,
}

impl Fonts {
    pub fn new(asset_server: &AssetServer) -> Self {
        Self {
            body: TextFont {
                font: FontSource::Handle(asset_server.load("EBGaramond-VariableFont_wght.ttf")),
                font_size: FontSize::Px(50.),
                weight: FontWeight::NORMAL,
                ..default()
            },
        }
    }
    pub fn body(&self, text: impl Into<String>) -> impl Bundle {
        (self.body.clone(), Text::new(text))
    }
}
