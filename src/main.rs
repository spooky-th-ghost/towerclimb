use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, move_camera, add_gravity))
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component, Default)]
pub struct Gravity(f32);

impl Gravity {
    pub fn new(value: f32) -> Self {
        Gravity(value)
    }

    pub fn add(&mut self, value: f32){
        self.0 += value;
    }

    pub fn reset(&mut self){
        self.0 = 0.0;
    }

    pub fn get(&self) -> f32{
        self.0
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let starting_translation = Vec3::new(-150.0, -120.0, 0.0);

    for i in 0..5 {
        commands.spawn((SpriteBundle {
            texture: asset_server.load("Block_basic.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            transform:Transform::from_translation(starting_translation + (Vec3::X*(i * 100) as f32)),
            ..default()
        },
        Collider::cuboid(50.0,50.0),
        RigidBody::Fixed
    ));
    }

    

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
        RigidBody::KinematicPositionBased,
        Collider::ball(25.0),
        KinematicCharacterController::default(),
        Gravity::default()
    ));
}

fn move_player(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut KinematicCharacterController, &mut Sprite, &Gravity), With<Player>>,
) {
    for (mut controller, mut sprite, gravity) in &mut player_query {
        let mut x_speed = 0.0;
        if input.pressed(KeyCode::D) {
            x_speed = 100.0 * time.delta_seconds();
            sprite.flip_x = false;
        }
        if input.pressed(KeyCode::A) {
            x_speed = -100.0 * time.delta_seconds();
            sprite.flip_x = true;
        }

        let movement_vector = Vec2::X * x_speed;

        let gravity_vector = Vec2::NEG_Y * gravity.get();
        println!("gravity value:{}",gravity.get());

        controller.translation = Some(movement_vector + gravity_vector); 
    }
}

fn move_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let mut camera_transform = camera_query.single_mut();
    let player_transform = player_query.single();
    camera_transform.translation.x = player_transform.translation.x;
}

fn add_gravity(
    time: Res<Time>, mut controller_query: Query<(&mut Gravity, &KinematicCharacterControllerOutput)> 

) {
    for (mut gravity, output) in &mut controller_query {
        if output.grounded{
            gravity.reset();
        }else{
            gravity.add(5.0 * time.delta_seconds());
        }
    }
}
