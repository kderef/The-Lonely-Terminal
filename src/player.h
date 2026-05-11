#pragma once

#include "raylib.h"
#include "src/player_camera.h"

typedef struct {
    union {
        Vector3 position;
        PlayerCamera camera;
    };
    float height;
    Vector3 velocity;
} Player;

Player player_new();

void player_update_camera(Player*);
void player_update_movement(Player*, bool freemove);
