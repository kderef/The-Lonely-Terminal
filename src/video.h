#pragma once

#include <raylib.h>

typedef struct {
    const char* title;
    int target_fps;
    
    int width;
    int height;

    bool vsync;
    bool fullscreen;
    bool msaa_4x;
    bool resizable;
} VideoConfig;

// Video video_new(VideoConfig);

void video_init(VideoConfig);
void video_close();

// video configuration at runtime

int video_set_fps(int);
bool video_toggle_fullscreen();
bool video_set_fullscreen(bool);

// video info

bool video_get_state(unsigned int flag);
void video_set_state(unsigned int flags);

bool video_is_fullscreen();
bool video_is_resizable();
bool video_is_msaa();
bool video_is_vsync();
bool video_is_minimized();
bool video_is_maximized();

int video_width();
int video_height();
Vector2 video_size();

// debug

void video_draw_debug();
