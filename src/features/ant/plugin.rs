use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use rand::prelude::*;

use bevy::window::PrimaryWindow;

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ant);
        app.add_systems(Update, movement_system);
    }
}

#[derive(Component)]
struct Ant;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

impl Velocity {
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    fn to_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

fn spawn_ant(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const TRIANGLE_SIZE: f32 = 6.0;
    const ANT_COUNT: usize = 1;

    let mesh = meshes.add(Triangle2d::new(
        Vec2::new(0.0, TRIANGLE_SIZE * 1.5),
        Vec2::new(-TRIANGLE_SIZE, -TRIANGLE_SIZE),
        Vec2::new(TRIANGLE_SIZE, -TRIANGLE_SIZE),
    ));

    let material = materials.add(Color::srgb(1.0, 0.0, 0.0));

    commands.spawn_batch((0..ANT_COUNT).map(move |_| {
        (
            Ant,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::default(),
            Velocity::zero(),
        )
    }));
}

fn movement_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_q.single().unwrap();
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let mut rng = rand::rng();

    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.x += rng.random_range(-0.3..0.3);
        velocity.y += rng.random_range(-0.3..0.3);

        velocity.x = velocity.x.clamp(-2.0, 2.0);
        velocity.y = velocity.y.clamp(-2.0, 2.0);

        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;

        transform.rotation = Quat::from_rotation_z(velocity.to_vec2().to_angle() - FRAC_PI_2);

        respawn_edges(&mut transform.translation.x, half_w);
        respawn_edges(&mut transform.translation.y, half_h);
    }
}

fn respawn_edges(value: &mut f32, max: f32) {
    if *value > max {
        *value = -max;
    } else if *value < -max {
        *value = max;
    }
}
