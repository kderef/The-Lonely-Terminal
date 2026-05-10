#include "player_camera.h"
#include "rcamera.h"

#include <math.h>
#include <raylib.h>
#include <raymath.h>

#include <stdint.h>

PlayerCamera player_camera_new() {
    PlayerCamera pc = {
        .fovy = 75.0,
        .position = {1, 0, 1},
        .projection = CAMERA_PERSPECTIVE,
        .target = {2, 1, 2},
        .up = {0, 1, 0},

        .mouse_grabbed = false,

        .yaw = 0,
        .pitch = 0,
        .sensitivity = 100,
    };

    return pc;
}

void player_camera_set_grab(PlayerCamera* pc, bool grab) {
    if (grab == pc->mouse_grabbed) return;

    if (grab) {
        HideCursor();
        DisableCursor();
    }
    else {
        ShowCursor();
        EnableCursor();
    }

    pc->mouse_grabbed = grab;
}

void player_camera_update(PlayerCamera* c) {
    const Vector2 m_delta = GetMouseDelta();    

    const float sens = c->sensitivity * 0.00005;

    c->yaw   -= m_delta.x * sens;
    c->pitch -= m_delta.y * sens;

    c->pitch = Clamp(c->pitch, -1.5, 1.5);

    // forward
    const Vector3 forward = (Vector3) {
        cosf(c->pitch) * sinf(c->yaw),
        sinf(c->pitch),
        cosf(c->pitch) * cosf(c->yaw)
    };

    // set target (mouse target)
    c->target = Vector3Add(c->position, forward);
}
