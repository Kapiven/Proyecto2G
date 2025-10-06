use bevy::{prelude::*, window::Cursor};

mod materials;
mod scene;
mod player;

use materials::*;
use scene::*;
use player::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Diorama Japonés 3D".to_string(),
                resolution: (1280., 720.).into(),
                cursor: Cursor {
                    visible: false,
                    grab_mode: bevy::window::CursorGrabMode::Locked,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        // Mejoras de renderizado y fondo
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::rgb(0.03, 0.03, 0.05)))
        .insert_resource(AmbientLight {
            color: Color::rgb(0.6, 0.6, 0.7),
            brightness: 0.4,
        })
        // Insertar materiales antes que cualquier Startup system
        .add_systems(PreStartup, add_materials)
        // Configurar la escena 
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, camera_movement)
        .add_systems(Update, rotate_diorama)
        .run();
}

// Sistema para agregar materiales
fn add_materials(
    mut commands: Commands,
    mut mats: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // Utilidades para crear texturas procedurales RGBA8 SRGB
    fn make_image(width: u32, height: u32, mut f: impl FnMut(u32, u32) -> [u8; 4]) -> Image {
        let mut data = Vec::with_capacity((width * height * 4) as usize);
        for y in 0..height { for x in 0..width { data.extend_from_slice(&f(x, y)); } }
        Image::new_fill(
            Extent3d { width, height, depth_or_array_layers: 1 },
            TextureDimension::D2,
            &data,
            TextureFormat::Rgba8UnormSrgb,
        )
    }
    // Césped: damero verde con variación
    let grass_img = make_image(128, 128, |x, y| {
        let s = 16; let cx = (x / s) % 2; let cy = (y / s) % 2;
        let base = if (cx ^ cy) == 0 { [46, 140, 64] } else { [38, 120, 56] };
        [base[0], base[1], base[2], 255]
    });
    let grass_h = images.add(grass_img);

    // Madera: vetas con bandas verticales
    let wood_img = make_image(128, 128, |x, y| {
        let t = ((x as f32 / 8.0).sin() * 0.5 + 0.5) * 40.0;
        let base = [110u8, 72, 45];
        let c = [
            base[0].saturating_add(t as u8),
            base[1].saturating_add((t * 0.7) as u8),
            base[2].saturating_add((t * 0.4) as u8),
        ];
        [c[0], c[1], c[2], 255]
    });
    let wood_h = images.add(wood_img);

    // Piedra: moteado con hash pseudoaleatorio por celda
    fn hash(u: u32) -> u32 { let mut v = u.wrapping_mul(747796405).wrapping_add(2891336453); v ^= v >> 16; v = v.wrapping_mul(2246822519); v ^ (v >> 13) }
    let stone_img = make_image(128, 128, |x, y| {
        let h = hash(x * 374761393 ^ y * 668265263);
        let n = (h & 0xFF) as u8; // 0..255
        let g = 120u8 + (n / 6);
        [g, g, g, 255]
    });
    let stone_h = images.add(stone_img);

    // Agua: azul con patrón ondulado suave
    let water_img = make_image(128, 128, |x, y| {
        let fx = x as f32 / 128.0; let fy = y as f32 / 128.0;
        let w = (((fx * 10.0).sin() + (fy * 14.0).cos()) * 0.5 + 0.5) * 30.0;
        let base = [40u8, 120, 200];
        [base[0], base[1].saturating_add(w as u8), base[2].saturating_add((w * 0.8) as u8), 160]
    });
    let water_h_img = images.add(water_img);

    // Metal: leve ruido lineal
    let metal_img = make_image(64, 64, |x, _y| {
        let v = 200u8.saturating_sub(((x % 8) as u8) * 4);
        [v, v, v, 255]
    });
    let metal_h = images.add(metal_img);

    let materials = Materials {
        // Césped
        grass: mats.add(StandardMaterial {
            base_color: Color::rgb(0.18, 0.55, 0.25),
            base_color_texture: Some(grass_h),
            perceptual_roughness: 0.9,
            metallic: 0.0,
            reflectance: 0.02,
            ..default()
        }),
        // Madera
        wood: mats.add(StandardMaterial {
            base_color: Color::rgb(0.43, 0.28, 0.16),
            base_color_texture: Some(wood_h),
            perceptual_roughness: 0.8,
            metallic: 0.0,
            reflectance: 0.04,
            ..default()
        }),
        // Piedra
        stone: mats.add(StandardMaterial {
            base_color: Color::rgb(0.5, 0.5, 0.5),
            base_color_texture: Some(stone_h),
            perceptual_roughness: 0.95,
            metallic: 0.0,
            reflectance: 0.03,
            ..default()
        }),
        // Vidrio (transparente)
        glass: mats.add(StandardMaterial {
            base_color: Color::rgba(0.8, 0.95, 1.0, 0.2),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.02,
            metallic: 0.0,
            reflectance: 0.08,
            ..default()
        }),
        // Agua (transparente, lisa)
        water: mats.add(StandardMaterial {
            base_color: Color::rgba(0.2, 0.5, 1.0, 0.4),
            base_color_texture: Some(water_h_img),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.03,
            metallic: 0.0,
            reflectance: 0.2,
            ..default()
        }),
        // Metal pulido (reflectante especular)
        metal: mats.add(StandardMaterial {
            base_color: Color::rgb(0.8, 0.82, 0.85),
            base_color_texture: Some(metal_h),
            metallic: 0.95,
            perceptual_roughness: 0.15,
            reflectance: 0.5,
            ..default()
        }),
        // Vidrio de linterna
        lantern_glass: mats.add(StandardMaterial {
            base_color: Color::rgba(1.0, 0.95, 0.8, 0.35),
            alpha_mode: AlphaMode::Blend,
            perceptual_roughness: 0.05,
            emissive: Color::rgb(1.0, 0.9, 0.6) * 0.3,
            ..default()
        }),
        // Cielo
        sky: mats.add(StandardMaterial {
            base_color: Color::rgb(0.52, 0.75, 0.95),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        }),
    };
    commands.insert_resource(materials);
}
