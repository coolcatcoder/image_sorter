use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, Images::insert.in_set(Setup));
}
crate::system_set!(Setup);

#[derive(Resource)]
pub struct Images(pub Vec<Handle<Image>>);

impl Images {
    fn insert(mut commands: Commands, asset_server: Res<AssetServer>) {
        let temporary_image = asset_server.load("IMG_6094.JPG");
        // let temporary_image = asset_server.add(Image::default());
        let images = Images(std::iter::repeat_n(temporary_image, 6).collect());
        commands.insert_resource(images);
    }
}
