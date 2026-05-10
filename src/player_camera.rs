use macroquad::prelude::*;

const MOVE_SPEED: f32 = 0.5;
const LOOK_SPEED: f32 = 200.0;

#[derive(Debug)]
pub struct PlayerCamera {
    position: Vec3,
    front: Vec3,
    right: Vec3,
    up: Vec3,
    yaw: f32,
    pitch: f32,
    world_up: Vec3,
    grabbed: bool,
}

impl PlayerCamera {
    pub fn new(position: Vec3) -> Self {
        let world_up = vec3(0.0, 1.0, 0.0);
        let yaw: f32 = 0.0;
        let pitch: f32 = 0.0;

        let front = vec3(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        )
        .normalize();
        let right = front.cross(world_up).normalize();
        let up = right.cross(front).normalize();

        Self {
            position,
            front,
            right,
            up,
            yaw,
            pitch,
            world_up,
            grabbed: false,
        }
    }

    pub fn update(&mut self, delta: f32) {
        // Movement
        if is_key_down(KeyCode::W) {
            self.position += self.front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            self.position -= self.front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.position -= self.right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.position += self.right * MOVE_SPEED;
        }

        // Mouse look
        // let _mouse_position: Vec2 = mouse_position().into();
        let mouse_delta = mouse_delta_position();

        if self.grabbed {
            self.yaw -= mouse_delta.x * delta * LOOK_SPEED;
            self.pitch += mouse_delta.y * delta * LOOK_SPEED;
            self.pitch = self.pitch.clamp(-1.5, 1.5);

            self.front = vec3(
                self.yaw.cos() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.sin() * self.pitch.cos(),
            )
            .normalize();

            self.right = self.front.cross(self.world_up).normalize();
            self.up = self.right.cross(self.front).normalize();
        }
        println!("{:?}", self.front);
    }

    pub fn toggle_grab(&mut self) {
        self.grabbed = !self.grabbed;
        set_cursor_grab(self.grabbed);
        show_mouse(!self.grabbed);
    }

    pub fn apply(&self) {
        set_camera(&Camera3D {
            position: self.position,
            up: self.up,
            target: self.position + self.front,
            fovy: 80.0_f64.to_radians() as f32,
            ..Default::default()
        });
    }
}
