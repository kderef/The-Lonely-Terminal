#include "menu.h"
#include "raylib.h"

PauseMenu pause_menu_new() {
    return (PauseMenu) {
        .paused = false
    };
}

bool pause_menu_toggle(PauseMenu* pm) {
    pm->paused = !pm->paused;

    return pm->paused;
}


void pause_menu_draw(PauseMenu* pm) {
    if (!pm->paused) return;

    int screen_w = GetScreenWidth();
    int screen_h = GetScreenHeight();

    const Color OVERLAY = {0, 0, 0, 100};
    DrawRectangle(0, 0, screen_w, screen_h, OVERLAY);
}
