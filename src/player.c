#include "player.h"
#include "raylib.h"
#include "rcamera.h"
#include "src/player_camera.h"

#include <raymath.h>
#include <stdio.h>


Player player_new() {
    Player p = {
        .camera = player_camera_new((Vector3){1, 1, 1}),
        .velocity = {0},
        .height = 1.0,
    };

    return p;
}

void player_update_camera(Player* p) {
    player_camera_update(&p->camera);
}

void player_update_movement(Player* p, bool freemove) {
    if (freemove) {
        float dt = GetFrameTime();

        const float MOVE_SPEED = 5.0;
        bool move_in_world_plane = false;
      
        // WASD
        float distance = MOVE_SPEED * dt;
    
        if (IsKeyDown(KEY_W)) CameraMoveForward(&p->camera.rcamera, distance, move_in_world_plane);
        if (IsKeyDown(KEY_S)) CameraMoveForward(&p->camera.rcamera, -distance, move_in_world_plane);
        if (IsKeyDown(KEY_D)) CameraMoveRight(&p->camera.rcamera, distance, move_in_world_plane);
        if (IsKeyDown(KEY_A)) CameraMoveRight(&p->camera.rcamera, -distance, move_in_world_plane);
    }
    else {
        // TODO: movement
        Vector3 forward = Vector3Normalize((Vector3){
            p->camera.target.x -  p->position.x,
            0.0f,
            p->camera.target.z - p->position.z
        });

        Vector3 right = {-forward.z, 0.0f, forward.x};

        const float move_speed = 0.03;

        const Vector3 move_speed_v3 = {move_speed, move_speed, move_speed};

        Vector3 forward2 = Vector3Multiply(forward, move_speed_v3);
        Vector3 right2 = Vector3Multiply(right, move_speed_v3);

        // Basic movement
        if (IsKeyDown(KEY_W)) {
            p->position = Vector3Add(p->position, forward2);
            p->camera.target = Vector3Add(p->camera.target, forward2);
        }
        if (IsKeyDown(KEY_S)) {
            p->position = Vector3Subtract(p->position, forward2);
            p->camera.target = Vector3Subtract(p->camera.target, forward2);
        }
        if (IsKeyDown(KEY_A)) {
            p->position = Vector3Subtract(p->position, right2);
            p->camera.target = Vector3Subtract(p->camera.target, right2);
        }
        if (IsKeyDown(KEY_D)) {
            p->position = Vector3Add(p->position, right2);
            p->camera.target = Vector3Add(p->camera.target, right2);
        }

        // Jumping & Gravity

        const float GRAVITY = -12.0f;
        const float JUMP_FORCE = 4.0f;

        float dt = GetFrameTime();
        float ground_y = 0.0f;
        bool grounded = (p->position.y - p->height) <= ground_y;

        if (IsKeyPressed(KEY_SPACE) && grounded) {
            p->velocity.y = JUMP_FORCE;
        }

        // Always apply gravity
        p->velocity.y += GRAVITY * dt;
        p->position.y += p->velocity.y * dt;

        // Ground clamp
        if (p->position.y - p->height <= ground_y) {
            p->position.y = ground_y + p->height;
            p->velocity.y = 0.0f;
        }

        // Sync camera to position
        p->camera.position.y = p->position.y;
        p->camera.target.y += p->velocity.y * dt;
    }
}
