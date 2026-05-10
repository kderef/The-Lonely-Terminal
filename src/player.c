#include "player.h"
#include "rcamera.h"
#include "src/player_camera.h"

Player player_new() {
    Player p = {
        .camera = player_camera_new(),
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
    }
}
