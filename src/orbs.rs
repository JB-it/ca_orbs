use std::f32::consts::E;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::settings::Settings;

#[derive(Component, PartialEq)]
enum Orb {
    Red,
    Green,
    Blue,
}

impl Orb {
    pub fn color(&self) -> Color {
        match self {
            Orb::Red => Color::rgb(1., 0., 0.),
            Orb::Green => Color::rgb(0., 1., 0.),
            Orb::Blue => Color::rgb(0., 0., 1.),
        }
    }

    pub fn get_attraction(&self, other: &Orb) -> f32 {
        match(self, other) {
            (Orb::Red, Orb::Green) => 1.,
            (Orb::Green, Orb::Blue) => 1.,
            (Orb::Blue, Orb::Red) => 1.,
            _ => {
                if self == other {
                    1.
                } else {
                    -1.
                }
            },
        }
    }
}

#[derive(Component, Clone)]
struct Rigidbody {
    velocity: Vec3,
    acceleration: Vec3,
    mass: f32,
}

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(orb_movement)
            .add_system(rigidbody_movement);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    for _i in 0..200 {
        let x_pos = rand::random::<f32>() * width - width / 2.;
        let y_pos = rand::random::<f32>() * height - height / 2.;

        let orb_type = match rand::random::<f32>() {
            x if x < 0.33 => Orb::Red,
            x if x < 0.66 => Orb::Green,
            _ => Orb::Blue,
        };

        commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(10.))
                    .with_translation(Vec3::new(x_pos, y_pos, 1.)),
                material: materials.add(ColorMaterial::from(orb_type.color())),
                ..default()
            }).insert(orb_type
            ).insert(Rigidbody {
                velocity: Vec3::new(0., 0., 0.),
                acceleration: Vec3::new(0., 0., 0.),
                mass: 1.,
            });
    }
}

fn orb_movement(
    mut orbs: Query<(&mut Transform, &mut Rigidbody, &Orb)>,
    time: Res<Time>,
    settings: Res<Settings>,
) {
    let mut iter = orbs.iter_combinations_mut();
    while let Some([(transform_i, mut rigidbody_i, orb_i), (transform_j, mut rigidbody_j, orb_j)]) =
        iter.fetch_next()
    {
        let distance = transform_i.translation.distance(transform_j.translation);
        
        if distance > settings.influence_distance {
            continue;
        }
        
        if distance > 10.0 {
            let force_i = settings.G * orb_i.get_attraction(orb_j) *rigidbody_i.mass *rigidbody_j.mass / (distance * settings.scale).powf(2.);
            let direction_i = transform_j.translation - transform_i.translation;
            let force_vector_i = direction_i.normalize() * force_i as f32;
            rigidbody_i.acceleration += force_vector_i * time.delta_seconds();
            
            let force_j = settings.G * orb_j.get_attraction(orb_i) *rigidbody_i.mass *rigidbody_j.mass / (distance * settings.scale).powf(2.);
            let direction_j = transform_j.translation - transform_i.translation;
            let force_vector_j = direction_j.normalize() * force_j as f32;
            rigidbody_j.acceleration -= force_vector_j * time.delta_seconds();
        } else {
            let force = -E.powf(5. - (distance * settings.scale) / 2.);
            let direction = transform_j.translation - transform_i.translation;
            let force_vector = direction.normalize() * force as f32;
            rigidbody_i.acceleration += force_vector * time.delta_seconds();
            rigidbody_j.acceleration -= force_vector * time.delta_seconds();
        }
    }
}

fn rigidbody_movement(
    mut orbs: Query<(&mut Transform, &mut Rigidbody)>,
    time: Res<Time>,
    settings: Res<Settings>,
    windows: Res<Windows>,
) {
    let width = windows.get_primary().unwrap().width();
    let height = windows.get_primary().unwrap().height();

    for (mut transform, mut rigidbody) in orbs.iter_mut() {
        let rb = rigidbody.clone();
        rigidbody.velocity += rb.acceleration;
        transform.translation += rigidbody.velocity;

        rigidbody.velocity.x = rigidbody.velocity.x * (1. - settings.friction);
        rigidbody.velocity.y = rigidbody.velocity.y * (1. - settings.friction);

        if settings.wraparound {
            transform.translation.x = (transform.translation.x + width * 1.5) % width - width / 2.;
            transform.translation.y = (transform.translation.y + height * 1.5) % height - height / 2.;
        } else {
            if transform.translation.x < -width / 2. {
                transform.translation.x = -width / 2. - (transform.translation.x + width / 2.);
                rigidbody.velocity.x = -rigidbody.velocity.x;
            }
            else if transform.translation.x > width / 2. {
                transform.translation.x = width / 2. - (transform.translation.x - width / 2.);
                rigidbody.velocity.x = -rigidbody.velocity.x;
            }
            if transform.translation.y < -height / 2. {
                transform.translation.y = -height / 2. - (transform.translation.y + height / 2.);
                rigidbody.velocity.y = -rigidbody.velocity.y;
            }
            else if transform.translation.y > height / 2. {
                transform.translation.y = height / 2. - (transform.translation.y - height / 2.);
                rigidbody.velocity.y = -rigidbody.velocity.y;
            }
        }

        rigidbody.acceleration = Vec3::new(0., 0., 0.);
    }
}