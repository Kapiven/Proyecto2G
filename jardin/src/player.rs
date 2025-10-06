use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};

#[derive(Component)]
pub struct PlayerCamera {
    pub speed: f32,
    pub sensitivity: f32, // grados por pixel
    pub pitch: f32,       // grados
    pub yaw: f32,         // grados
    pub zoom_speed: f32,  // grados por unidad de scroll
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 15.0)
                .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                near: 0.1,
                far: 1000.0,
                ..default()
            }),
            ..default()
        },
        PlayerCamera { speed: 10.0, sensitivity: 0.1, pitch: 0.0, yaw: -90.0, zoom_speed: 2.0 },
    ));
}

pub fn camera_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut PlayerCamera, &mut Projection), With<Camera>>,
) {
    if let Ok((mut transform, mut player, mut projection)) = query.get_single_mut() {
        // Rotación con mouse (yaw/pitch)
        let mut delta = Vec2::ZERO;
        for ev in mouse_motion_events.iter() {
            delta += ev.delta;
        }
        if delta.length_squared() > 0.0 {
            player.yaw += delta.x * player.sensitivity;
            player.pitch = (player.pitch - delta.y * player.sensitivity).clamp(-89.0, 89.0);

            let yaw_r = player.yaw.to_radians();
            let pitch_r = player.pitch.to_radians();
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_r, pitch_r, 0.0);
        }

        // Movimiento WASD + vuelo
        let mut direction = Vec3::ZERO;
        let forward = transform.forward();
        let right = transform.right();
        if keys.pressed(KeyCode::W) { direction += forward; }
        if keys.pressed(KeyCode::S) { direction -= forward; }
        if keys.pressed(KeyCode::A) { direction -= right; }
        if keys.pressed(KeyCode::D) { direction += right; }
        if keys.pressed(KeyCode::Space) { direction += Vec3::Y; }
        if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) { direction -= Vec3::Y; }

        if direction.length_squared() > 0.0 {
            let speed = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                player.speed * 2.0
            } else {
                player.speed
            };
            transform.translation += direction.normalize() * speed * time.delta_seconds();
        }

        // Zoom con la rueda del mouse (ajustar FOV)
        let mut scroll_accum = 0.0f32;
        for ev in mouse_wheel_events.iter() {
            let mut s = ev.y as f32;
            if let MouseScrollUnit::Pixel = ev.unit { s /= 50.0; }
            scroll_accum += s;
        }
        if scroll_accum.abs() > 0.0 {
            if let Projection::Perspective(ref mut persp) = *projection {
                let delta_fov = scroll_accum * player.zoom_speed;
                let new_fov = (persp.fov - delta_fov.to_radians()).clamp(20f32.to_radians(), 90f32.to_radians());
                persp.fov = new_fov;
            }
        }
    }
}
