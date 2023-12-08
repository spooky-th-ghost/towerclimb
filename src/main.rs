use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn(Camera2dBundle::default());

    // Spawn our guy
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tower_guy.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            ..default()
        },
        Player,
    ));
}
