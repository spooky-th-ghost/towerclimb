use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_camera))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera
));

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
    ));
}



fn move_player(input: Res<Input<KeyCode>>, time: Res<Time>, mut player_query: Query<(&mut Transform, &mut Sprite), With<Player>>){
    for (mut transform, mut sprite) in &mut player_query{
        let mut x_speed = 0.0; 
        if input.pressed(KeyCode::D){
            x_speed = 100.0*time.delta_seconds();
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::A){
            x_speed = -100.0*time.delta_seconds();
            sprite.flip_x = true;

        }
        transform.translation += Vec3::X*x_speed;
    }
}

fn move_camera(mut camera_query: Query<&mut Transform, With<MainCamera>>, player_query: Query<&Transform,(With<Player>, Without<MainCamera>)>){
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation.x = player_transform.translation.x;
}

