#pragma once

#include <raylib.h>

typedef struct {
    Mesh cube;
    Model model;
} Skybox;

// Generate a skybox struct from a cubemap image
Skybox skybox_generate(Texture2D);
void skybox_unload(Skybox*);
void skybox_draw(Skybox*);
