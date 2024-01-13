use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub mod combat;
pub mod enemy;
pub mod physics;
pub mod player;

fn main() {
    App::new()
        // Add the default plugins, basically you will do this for 100% of bevy projects
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Add our physics plugin
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // Add a debug plugin that renders our physics colliders
        .add_plugins(RapierDebugRenderPlugin::default())
        // A debug plugin that allows us to view the components attached to each entity
        .add_plugins(WorldInspectorPlugin::default())
        // Set the strength of our gravity
        .insert_resource(RapierConfiguration {
            gravity: Vec2::NEG_Y * 1000.0,
            ..default()
        })
        // Add our `setup` system to the Startup stage, so that it runs one time when the app
        // starts up
        .add_systems(Startup, setup)
        // Add the rest of our systems to the Update stage so that they run every tick/frame that
        // the application is running
        .add_plugins((
            combat::CombatPlugin,
            physics::PhysicsPlugin,
            player::PlayerPlugin,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the camera
    commands.spawn((Camera2dBundle::default(), player::MainCamera));

    // Starting position for the platforms
    let starting_translation = Vec3::new(-150.0, -120.0, 0.0);

    let terrain_groups = CollisionGroups::new(
        Group::from_bits_truncate(0b0001),
        Group::from_bits_truncate(0b0111),
    );

    // Instead of copy pasting the platform spawn a bunch we just use a for loop here
    for i in 0..50 {
        commands.spawn((
            SpriteBundle {
                // Load the block sprite from out ./assets folder
                texture: asset_server.load("Block_basic.png"),
                sprite: Sprite {
                    // Set the sprite to be displayed at 100x100 pixels instead of it's native
                    // resolution
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..default()
                },
                // Set the position for the block
                transform: Transform::from_translation(
                    starting_translation + (Vec3::X * (i * 100) as f32),
                ),
                ..default()
            },
            terrain_groups,
            // Give the block a collider so that it can be stood on
            Collider::cuboid(50.0, 50.0),
            // Give the block a fixed rigid body, meaning it won't move but can be collided with
            RigidBody::Fixed,
        ));
    }

    commands.spawn((
        SpriteBundle {
            // Load the block sprite from out ./assets folder
            texture: asset_server.load("Block_basic.png"),
            sprite: Sprite {
                // Set the sprite to be displayed at 100x100 pixels instead of it's native
                // resolution
                custom_size: Some(Vec2::new(500., 100.)),
                ..default()
            },
            // Set the position for the block
            transform: Transform::from_translation(Vec3::new(-1000.0, -100.0, 0.0))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, 30.0_f32.to_degrees())),
            ..default()
        },
        // Give the block a collider so that it can be stood on
        Collider::cuboid(250.0, 50.0),
        // Give the block a fixed rigid body, meaning it won't move but can be collided with
        RigidBody::Fixed,
        terrain_groups,
        Name::from("Ramp"),
    ));
}
