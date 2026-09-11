pub use bevy::prelude::*;
use bevy::{
    camera::RenderTarget,
    window::{Monitor, PrimaryMonitor, WindowRef},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_windows.in_set(Setup));
}
crate::system_set!(Setup);

#[derive(Component)]
pub struct PrimaryCamera;

fn create_windows(
    mut commands: Commands,
    primary_monitor: Single<Entity, With<PrimaryMonitor>>,
    secondary_monitors: Query<Entity, (With<Monitor>, Without<PrimaryMonitor>)>,
) {
    let mut create_window = |monitor, primary| {
        let mut window = Window {
            position: WindowPosition::Centered(MonitorSelection::Entity(monitor)),
            ..default()
        };
        window.set_maximized(true);

        let window = commands
            .spawn(window)
            .observe(
                |_: On<Despawn, Window>, mut app_exit_writer: MessageWriter<AppExit>| {
                    app_exit_writer.write(AppExit::Success);
                },
            )
            .id();

        let mut camera =
            commands.spawn((Camera2d, RenderTarget::Window(WindowRef::Entity(window))));
        if primary {
            camera.insert(PrimaryCamera);
        }
    };

    create_window(*primary_monitor, true);

    if let Some(secondary_monitor) = secondary_monitors.iter().next() {
        create_window(secondary_monitor, false);
    }
}
