use bevy::prelude::*;
use crate::materials::Materials;

#[derive(Component)]
pub struct DioramaRoot;

pub fn setup_scene(
    mut commands: Commands,
    materials: Res<Materials>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Nodo raíz del diorama
    let root = commands.spawn((SpatialBundle::default(), DioramaRoot)).id();

    // Suelo de césped
    let ground = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 60.0, subdivisions: 1 })),
        material: materials.grass.clone(),
        ..default()
    }).id();
    commands.entity(root).add_child(ground);

    // Sendero de piedra (doble hilera, ligeramente curvada)
    for i in -15..=15 {
        let z = i as f32 * 1.2;
        for x in [-0.7_f32, 0.7_f32] {
            let stone = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.stone.clone(),
                transform: Transform {
                    translation: Vec3::new(x + (i as f32 * 0.05).sin() * 0.3, 0.05, z),
                    scale: Vec3::new(1.2, 0.1, 0.8),
                    ..default()
                },
                ..default()
            }).id();
            commands.entity(root).add_child(stone);
        }
    }

    // Estanque rectangular de agua con bordes de piedra
    let pond_center = Vec3::new(-6.0, 0.1, -3.0);
    let pond_size = Vec3::new(6.0, 0.2, 4.0);
    let pond = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.water.clone(),
        transform: Transform {
            translation: pond_center,
            scale: pond_size,
            ..default()
        },
        ..default()
    }).id();
    commands.entity(root).add_child(pond);

    // Borde de piedra alrededor del estanque
    let border_thickness = 0.3;
    let bx = pond_size.x + border_thickness;
    let bz = pond_size.z + border_thickness;
    let by = 0.4;
    for &(dx, dz, sx, sz) in &[
        (0.0, -bz / 2.0, bx, border_thickness),
        (0.0,  bz / 2.0, bx, border_thickness),
        (-bx / 2.0, 0.0, border_thickness, bz),
        ( bx / 2.0, 0.0, border_thickness, bz),
    ] {
        let border = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.stone.clone(),
            transform: Transform {
                translation: pond_center + Vec3::new(dx, by, dz),
                scale: Vec3::new(sx, 0.4, sz),
                ..default()
            },
            ..default()
        }).id();
        commands.entity(root).add_child(border);
    }

    // Árboles sencillos 
    let tree_positions = [
        Vec3::new(6.0, 0.0, -6.0),
        Vec3::new(10.0, 0.0, 2.0),
        Vec3::new(-10.0, 0.0, 6.0),
        Vec3::new(0.0, 0.0, -10.0),
    ];
    for pos in tree_positions {
        // Tronco (cilindro)
        let trunk = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder { radius: 0.25, height: 2.0, resolution: 20, segments: 1 })),
            material: materials.wood.clone(),
            transform: Transform { translation: pos + Vec3::new(0.0, 1.0, 0.0), ..default() },
            ..default()
        }).id();
        commands.entity(root).add_child(trunk);
        // Copas (esferas)
        for (dy, scale) in [(2.2, 1.8), (3.0, 1.4), (3.6, 1.1)] {
            let crown = commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 16, stacks: 10 })),
                material: materials.grass.clone(),
                transform: Transform { translation: pos + Vec3::new(0.0, dy, 0.0), scale: Vec3::splat(scale), ..default() },
                ..default()
            }).id();
            commands.entity(root).add_child(crown);
        }
    }

    // Rocas decorativas (esferas deformadas)
    for (x, z, s) in [
        (-3.0, 4.0, 0.6),
        (-4.5, 5.2, 0.9),
        (-2.0, 3.2, 0.4),
        (3.0, -2.0, 0.7),
        (5.0, -4.0, 0.5),
    ] {
        let rock = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 0.5, sectors: 16, stacks: 10 })),
            material: materials.stone.clone(),
            transform: Transform {
                translation: Vec3::new(x, s * 0.5, z),
                scale: Vec3::new(s * 1.4, s, s),
                rotation: Quat::from_euler(EulerRot::XYZ, 0.2, 0.4, 0.1),
                ..default()
            },
            ..default()
        }).id();
        commands.entity(root).add_child(rock);
    }

    // Torii sencillo de madera cerca del sendero
    let gate_pos = Vec3::new(0.0, 0.0, -8.0);
    for x in [-1.2_f32, 1.2_f32] {
        let pillar = commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.wood.clone(),
            transform: Transform { translation: gate_pos + Vec3::new(x, 1.5, 0.0), scale: Vec3::new(0.3, 3.0, 0.3), ..default() },
            ..default()
        }).id();
        commands.entity(root).add_child(pillar);
    }
    let beam = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.wood.clone(),
        transform: Transform { translation: gate_pos + Vec3::new(0.0, 3.2, 0.0), scale: Vec3::new(3.2, 0.3, 0.5), ..default() },
        ..default()
    }).id();
    commands.entity(root).add_child(beam);

    // Linterna: base de madera + detalle metálico + caja de vidrio + luz puntual
    let lantern_pos = Vec3::new(2.8, 0.0, -2.5);
    let base = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.wood.clone(),
        transform: Transform { translation: lantern_pos + Vec3::new(0.0, 0.5, 0.0), scale: Vec3::new(0.3, 1.0, 0.3), ..default() },
        ..default()
    }).id();
    commands.entity(root).add_child(base);

    let metal = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.metal.clone(),
        transform: Transform { translation: lantern_pos + Vec3::new(0.0, 1.05, 0.0), scale: Vec3::new(0.4, 0.05, 0.4), ..default() },
        ..default()
    }).id();
    commands.entity(root).add_child(metal);

    let glass = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.lantern_glass.clone(),
        transform: Transform { translation: lantern_pos + Vec3::new(0.0, 1.35, 0.0), scale: Vec3::new(0.8, 0.8, 0.8), ..default() },
        ..default()
    }).id();
    commands.entity(root).add_child(glass);

    let light = commands.spawn(PointLightBundle {
        point_light: PointLight { intensity: 1400.0, range: 14.0, shadows_enabled: true, ..default() },
        transform: Transform::from_translation(lantern_pos + Vec3::new(0.0, 1.35, 0.0)),
        ..default()
    }).id();
    commands.entity(root).add_child(light);

    // Skybox simple: gran esfera invertida que envuelve la escena
    let sky = commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere { radius: 200.0, sectors: 32, stacks: 16 })),
        material: materials.sky.clone(),
        transform: Transform { translation: Vec3::ZERO, rotation: Quat::IDENTITY, scale: Vec3::new(-1.0, 1.0, 1.0), ..default() },
        ..default()
    }).id();
    commands.entity(root).add_child(sky);

    // Luz direccional (sol) fija en el mundo (no hija del diorama)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight { illuminance: 26000.0, shadows_enabled: true, ..default() },
        transform: Transform { rotation: Quat::from_euler(EulerRot::XYZ, -1.0, 0.7, 0.0), ..default() },
        ..default()
    });
}

// Rotación del diorama: sólo rota cuando se presionan Q/E
pub fn rotate_diorama(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<DioramaRoot>>,
) {
    let mut yaw_speed = 0.0_f32; // rad/s
    if keys.pressed(KeyCode::Q) { yaw_speed += 0.8; }
    if keys.pressed(KeyCode::E) { yaw_speed -= 0.8; }
    if yaw_speed == 0.0 { return; }
    let angle = yaw_speed * time.delta_seconds();
    if let Ok(mut t) = query.get_single_mut() {
        t.rotate_y(angle);
    }
}
