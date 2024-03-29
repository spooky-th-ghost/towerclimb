use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    combat::Kick,
    physics::{GroundSensor, Jump},
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawner::default())
            .add_systems(Update, spawn_enemy);
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    sprite_bundle: SpriteBundle,
    enemy: Enemy,
    rigidbody: RigidBody,
    collider: Collider,
    velocity: Velocity,
    locked_axes: LockedAxes,
    jump: Jump,
    kick: Kick,
    ground_sensor: GroundSensor,
    name: Name,
    collision_groups: CollisionGroups,
}

impl Default for EnemyBundle {
    fn default() -> Self {
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..default()
                },
                ..default()
            },
            enemy: Enemy,
            rigidbody: RigidBody::Dynamic,
            // Give the player a capsule shaped collider
            collider: Collider::capsule_y(30.0, 25.0),
            // Give the player a velocity component so that it can be moved by the physics system
            velocity: Velocity::default(),
            // Make it so that the player can not be rotated by physics interactions (so it doesn't
            // fall over while moveing)
            locked_axes: LockedAxes::ROTATION_LOCKED,
            // Give the player a jump component so that he can jump
            jump: Jump::new(550.0),
            // Give the player a kick component so that he can kick
            kick: Kick::new(1000.0),
            // Give the player a ground sensor so that it can detect that it's on the ground
            ground_sensor: GroundSensor::default(),
            name: Name::from("Enemy"),
            collision_groups: CollisionGroups::default(),
        }
    }
}

impl EnemyBundle {
    pub fn new(texture: Handle<Image>, size: Vec2, position: Vec3, color: Color) -> Self {
        EnemyBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(position),
                texture,
                sprite: Sprite {
                    color,
                    custom_size: Some(size),
                    ..Default::default()
                },
                ..Default::default()
            },
            collision_groups: CollisionGroups::new(
                Group::from_bits_truncate(0b1000),
                Group::from_bits_truncate(0b1111),
            ),
            ..Default::default()
        }
    }
}

// TODO: Make genric when it makes sense
#[derive(Resource)]
pub struct EnemySpawner {
    count: u8,
    spawn_time: Timer,
}

impl EnemySpawner {
    pub fn tick(&mut self, delta: std::time::Duration) {
        self.spawn_time.tick(delta);
    }

    pub fn should_spawn(&self) -> bool {
        self.spawn_time.just_finished()
    }

    pub fn spawn_enemy(&mut self) {
        self.count += 1;
        self.spawn_time.reset();
    }
}

impl Default for EnemySpawner {
    fn default() -> Self {
        EnemySpawner {
            count: 0,
            spawn_time: Timer::from_seconds(4.0, TimerMode::Repeating),
        }
    }
}
// TODO decide spawn location
fn spawn_enemy(
    mut commands: Commands,
    mut spawner: ResMut<EnemySpawner>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawner.tick(time.delta());
    if spawner.should_spawn() {
        let mut rng = rand::thread_rng();

        let random_number: f32 = rng.gen();
        let x_pos: f32 = (random_number * 300.0) + 300.0;

        let enemy_position = Vec3::new(x_pos, 50.0, 0.0);

        spawner.spawn_enemy();
        // TODO fix Colins BUll SHIT
        commands.spawn(EnemyBundle::new(
            asset_server.load("tower_guy.png"),
            Vec2::new(100.0, 100.0),
            enemy_position,
            Color::BLUE,
        ));
    }
}
