#pragma once

#include <raylib.h>

typedef struct {
    bool paused;
} PauseMenu;

PauseMenu pause_menu_new();

// Toggles the pause menu, returns the new state (on/off)
bool pause_menu_toggle(PauseMenu*);

void pause_menu_draw(PauseMenu*);
