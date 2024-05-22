use std::{collections::HashSet, ops::RangeInclusive};

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

#[derive(Component, Deref, DerefMut, Copy, Clone)]
struct HasColor(Color);

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let player_size = match args.get(1).map(|str| str.as_str()) {
        Some("broken") => 20.0,
        Some("working") => 19.0,
        _ => panic!("Pass either working on broken as the first argument"),
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.world.resource_mut::<RapierConfiguration>().gravity = Vec3::new(0.0, 0.0, 0.0);
    app.world.spawn(Camera2dBundle::default());
    app.add_systems(Update, (keyboard_movement, draw_body));

    // Floor
    spawn_grid_of_cubes(&mut app, -3..=3, -3..=3, -2.0, Color::GRAY);
    // Top wall
    spawn_grid_of_cubes(&mut app, -3..=3, 3..=3, 0.0, Color::BLUE);
    // Bottom wall
    spawn_grid_of_cubes(&mut app, -3..=3, -3..=-3, 0.0, Color::BLUE);
    // Right wall
    spawn_grid_of_cubes(&mut app, 3..=3, -3..=3, 0.0, Color::BLUE);
    // Left wall
    spawn_grid_of_cubes(&mut app, -3..=-3, -3..=3, 0.0, Color::BLUE);

    app.world
        .spawn_empty()
        .insert(RigidBody::Dynamic)
        .insert(HasColor(Color::GREEN))
        .insert(Name::new("Player"))
        .insert(Velocity::zero())
        .insert(Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::cuboid(player_size, player_size, 1.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::default());

    app.run();
}

fn spawn_grid_of_cubes(
    app: &mut App,
    range_x: RangeInclusive<i32>,
    range_y: RangeInclusive<i32>,
    z: f32,
    color: Color,
) {
    for x in range_x {
        for y in range_y.clone() {
            app.world
                .spawn_empty()
                .insert(RigidBody::Fixed)
                .insert(HasColor(color))
                .insert(Name::new("FixedCube"))
                .insert(Collider::cuboid(10.0, 10.0, 1.0))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(TransformBundle {
                    local: Transform::from_translation(Vec3::new(
                        x as f32 * 20.0,
                        y as f32 * 20.0,
                        z,
                    )),
                    ..default()
                });
        }
    }
}

fn draw_body(mut gizmos: Gizmos, characters: Query<(&GlobalTransform, &Collider, &HasColor)>) {
    for (transform, collider, &HasColor(color)) in characters.iter() {
        let cuboid = collider.as_cuboid().unwrap().half_extents().truncate();
        gizmos.rect(transform.translation(), Quat::IDENTITY, cuboid * 2.0, color);
    }
}

pub fn keyboard_movement(
    mut input: EventReader<KeyboardInput>,
    mut query: Query<&mut Velocity>,
    mut down_keys: Local<HashSet<KeyCode>>,
) {
    for event in input.read() {
        match event.state {
            ButtonState::Pressed => down_keys.insert(event.key_code),
            ButtonState::Released => down_keys.remove(&event.key_code),
        };
    }
    for mut velocity in query.iter_mut() {
        let mut new_velocity = Vec3::default();
        if down_keys.contains(&KeyCode::ArrowUp) {
            new_velocity.y += 200.0;
        }
        if down_keys.contains(&KeyCode::ArrowDown) {
            new_velocity.y -= 200.0;
        }
        if down_keys.contains(&KeyCode::ArrowRight) {
            new_velocity.x += 200.0;
        }
        if down_keys.contains(&KeyCode::ArrowLeft) {
            new_velocity.x -= 200.0;
        }
        velocity.linvel = new_velocity;
        println!("Velocity: {:?}", velocity.linvel);
    }
}
