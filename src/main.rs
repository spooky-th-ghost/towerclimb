use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, move_player)
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

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("tower_guy.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            transform:Transform::from_translation(Vec3::Y*150.0),
            ..default()
        },
        Player,
    ));
}



fn move_player(input: Res<Input<KeyCode>>, time: Res<Time>, mut player_query: Query<&mut Transform, With<Player>>){
    for mut transform in &mut player_query{
        let mut x_speed = 0.0; 
        if input.pressed(KeyCode::D){
            x_speed = 20.0*time.delta_seconds();
        }
        if input.pressed(KeyCode::A){
            x_speed = -20.0*time.delta_seconds();
        }
        transform.translation += Vec3::X*x_speed;
    }
}
