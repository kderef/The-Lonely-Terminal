#include "game.h"

#include "menu.h"
#include "player_camera.h"
#include "asset.h"

#include <time.h>
#include <raylib.h>

Game game_init(GameConfig conf) {
    Game g = {
        .config = conf,
        .running = true,
        .debug = true,
        .freemove = false,
        .pause_menu = pause_menu_new(),
        .player = player_new(),
    };
    
    return g;
}

void game_open(Game* g) {
    const GameConfig c = g->config;

    unsigned int flags = 0;

    if (c.fullscreen) flags |= FLAG_FULLSCREEN_MODE;
    if (c.msaa_4x) flags |= FLAG_MSAA_4X_HINT;
    if (c.vsync) flags |= FLAG_VSYNC_HINT;
    if (c.window_resizable) flags |= FLAG_WINDOW_RESIZABLE;

    SetConfigFlags(flags);

    // Open the window
    InitWindow(c.screen_w, c.screen_h, c.window_title);

    SetExitKey(0);
    SetTargetFPS(c.target_fps);

    // Audio init
    // audio_init(&g->audio)


    SetRandomSeed(time(0));

    // grab cursor
    player_camera_set_grab(&g->player.camera, true);
}

void game_load(Game* g) {
    Asset* sky1 = asset_load(AID_SKY1);
    
    g->skybox = skybox_generate(sky1->texture);
}

void game_close(Game* g) {
    asset_unload_all();
    
    skybox_unload(&g->skybox);

    player_camera_set_grab(&g->player.camera, false);
       
}

void game_update(Game* g) {
    if (WindowShouldClose()) g->running = false;

    bool paused = g->pause_menu.paused;

    if (IsKeyPressed(KEY_ESCAPE)) {
        paused = pause_menu_toggle(&g->pause_menu);
        player_camera_set_grab(&g->player.camera, !paused);
    }

    if (IsKeyPressed(KEY_F11)) {
        ToggleFullscreen();
    }
    if (IsKeyPressed(KEY_F1)) {
        g->debug = !g->debug;
    }
    if (IsKeyPressed(KEY_F2)) g->freemove = !g->freemove;

    if (!paused) {
        // UpdateCamera(&g->player_camera.camera, CAMERA_FIRST_PERSON);
        player_update_camera(&g->player);
        player_update_movement(&g->player, g->freemove);
    }
}

void game_draw(Game* g) {
    BeginDrawing();
    ClearBackground(BLACK);

    BeginMode3D(g->player.camera.rcamera);
    {
        skybox_draw(&g->skybox);
        
        DrawGrid(10, 1.0);

        DrawCube(
            (Vector3){0.0, 0.5, 0.0},
            0.5,
            0.5,
            0.5,
            RED
        );
    }
    EndMode3D();

    pause_menu_draw(&g->pause_menu);

    if (g->debug) {
        const Color COLOR = YELLOW;
        const int font_size = 30;
        int x = 5;
        int y = -font_size + 5;

        const char* text;

        #define DEBUG_LINE(...) DrawText((text = TextFormat(__VA_ARGS__)), x, y += font_size, font_size, COLOR)

        DEBUG_LINE("FPS: %d", GetFPS());
        DEBUG_LINE("resolution: %dx%d", GetScreenWidth(), GetScreenHeight());
        DEBUG_LINE("fullscreen: %d", IsWindowFullscreen());
        DEBUG_LINE("MSAA 4x: %d", IsWindowState(FLAG_MSAA_4X_HINT));
        DEBUG_LINE("vsync: %d", IsWindowState(FLAG_VSYNC_HINT));
        y += font_size;
        DEBUG_LINE("freemove: %d", g->freemove);
        DEBUG_LINE("player_velocity: (%f, %f, %f)", g->player.velocity.x, g->player.velocity.y, g->player.velocity.z);
        DEBUG_LINE("player_position: (%f, %f, %f)", g->player.position.x, g->player.position.y, g->player.position.z);

        #undef DEBUG_LINE
    }

    EndDrawing();
}

