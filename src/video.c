#include "video.h"

#include <raylib.h>


void video_init(VideoConfig config) {
    unsigned int flags = 0;

    if (config.msaa_4x) flags |= FLAG_MSAA_4X_HINT;
    if (config.resizable) flags |= FLAG_WINDOW_RESIZABLE;
    if (config.vsync) flags |= FLAG_VSYNC_HINT;
        
    // if(flags...)

    SetConfigFlags(flags);

    // Init window
    
    InitWindow(config.width, config.height, config.title);

    SetExitKey(0);
    SetTargetFPS(config.target_fps);

    if (config.fullscreen) video_set_fullscreen(true);
}

int video_set_fps(int fps_target) {
    SetTargetFPS(fps_target);
    return fps_target;
}

// fullscreen

bool video_toggle_fullscreen() {
    ToggleFullscreen();
    return IsWindowFullscreen();
}

bool video_set_fullscreen(bool fullscreen) {
    if (IsWindowFullscreen() != fullscreen) {
        ToggleFullscreen();
    }
    return fullscreen;
}

void video_close() {
    CloseWindow();
}

// state

bool video_get_state(unsigned int flag) {
    return IsWindowState(flag);
}

void video_set_state(unsigned int flags) {
    SetWindowState(flags);
}

// get info

bool video_is_fullscreen() {
    return IsWindowFullscreen();
}

bool video_is_resizable() {
    return IsWindowState(FLAG_WINDOW_RESIZABLE);
}
bool video_is_vsync() {
    return IsWindowState(FLAG_VSYNC_HINT);
}
bool video_is_msaa() {
    return IsWindowState(FLAG_MSAA_4X_HINT);
}
bool video_is_minimized() {
    return IsWindowMinimized();
}
bool video_is_maximized() {
    return IsWindowMaximized();
}

int video_width() {
    return GetScreenWidth();
}
int video_height() {
    return GetScreenHeight();
}
Vector2 video_size() {
    return (Vector2) { video_width(), video_height() };
}

/*********************************************************************************************/


void video_draw_debug() {
    const Color COLOR = YELLOW;
    const int font_size = 30;
    int x = 5;
    int y = -font_size + 5;

    const char* text;

    #define DEBUG_LINE(...) DrawText((text = TextFormat(__VA_ARGS__)), x, y += font_size, font_size, COLOR)

    DEBUG_LINE("FPS: %d", GetFPS());
    DEBUG_LINE("resolution: %dx%d", video_width(), video_height());
    DEBUG_LINE("fullscreen: %d", video_is_fullscreen());
    DEBUG_LINE("resizable: %d", video_is_resizable());
    DEBUG_LINE("MSAA 4x: %d", video_is_msaa());
    DEBUG_LINE("vsync: %d", video_is_vsync());

    #undef DEBUG_LINE
}
