use raylib::prelude::*;

const MOVE_SPEED: f32 = 0.1;
const LOOK_SPEED: f32 = 0.6;

pub fn set_grab(rl: &mut RaylibHandle, grab: bool) {
    if grab {
        rl.hide_cursor();
        rl.disable_cursor();
    } else {
        rl.show_cursor();
        rl.enable_cursor();
    }
}
pub fn toggle_grab(rl: &mut RaylibHandle, grab: &mut bool) {
    *grab = !*grab;
    set_grab(rl, *grab);
}

pub fn update_camera(rl: &RaylibHandle, camera: &mut Camera3D) {
    let forward = (camera.target - camera.position).normalized();
    let right = Vector3::new(0.0, 1.0, 0.0).cross(forward).normalized();

    // Movement
    if rl.is_key_down(KeyboardKey::KEY_W) {
        camera.position += forward * MOVE_SPEED;
        camera.target += forward * MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        camera.position -= forward * MOVE_SPEED;
        camera.target -= forward * MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        camera.position -= right * MOVE_SPEED;
        camera.target -= right * MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        camera.position += right * MOVE_SPEED;
        camera.target += right * MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_SPACE) {
        camera.position.y += MOVE_SPEED;
        camera.target.y += MOVE_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        camera.position.y -= MOVE_SPEED;
        camera.target.y -= MOVE_SPEED;
    }

    // Mouse look
    let mouse_delta = rl.get_mouse_delta();
    let yaw = mouse_delta.x * LOOK_SPEED * rl.get_frame_time();
    let pitch = mouse_delta.y * LOOK_SPEED * rl.get_frame_time();

    let mut front = (camera.target - camera.position).normalized();

    // Rotate yaw around world up
    front = Vector3::new(
        front.x * yaw.cos() - front.z * yaw.sin(),
        front.y,
        front.x * yaw.sin() + front.z * yaw.cos(),
    );

    // Rotate pitch around right axis
    let pitched = front * pitch.cos()
        + Vector3::new(0.0, 1.0, 0.0)
            .cross(front)
            .normalized()
            .cross(front)
            * pitch.sin();

    // Clamp pitch to avoid gimbal flip
    if pitched.y.abs() < 0.99 {
        front = pitched.normalized();
    }

    camera.target = camera.position + front;
}
