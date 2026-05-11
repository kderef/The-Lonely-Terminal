#pragma once

#include <raylib.h>
#include <stdint.h>

typedef struct {
    union {
        // Camera3D fields accessible
        struct {
            Vector3 position;       
            Vector3 target;         
            Vector3 up;             
            float fovy;             
            int projection;         
        };
        Camera3D rcamera;
    };

    float yaw;
    float pitch;
    uint8_t sensitivity;
    
    bool mouse_grabbed;
} PlayerCamera;

/**********************************************************************/

PlayerCamera player_camera_new(Vector3 position);
void player_camera_set_grab(PlayerCamera*, bool);

// Update the camera's target to mouse movement
void player_camera_update(PlayerCamera*);
