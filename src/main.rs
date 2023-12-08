use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
