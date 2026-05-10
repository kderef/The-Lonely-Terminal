#include "game.h"

#include "menu.h"
#include "player_camera.h"
#include "asset.h"
#include "video.h"

#include <time.h>
#include <raylib.h>

Game game_init(GameConfig conf) {
    Game g = {
        .config = conf,
        .running = true,
        .debug =true,
        .pause_menu = pause_menu_new(),
        .player = player_new(),
        .audio = audio_new(),
    };
    
    return g;
}

void game_open(Game* g) {
    // Video init
    video_init(g->config.video_conf);

    // Audio init
    audio_init(&g->audio);


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
        
    audio_close(&g->audio);
    video_close();
}

void game_update(Game* g) {
    if (WindowShouldClose()) g->running = false;

    bool paused = g->pause_menu.paused;

    if (IsKeyPressed(KEY_ESCAPE)) {
        paused = pause_menu_toggle(&g->pause_menu);
        player_camera_set_grab(&g->player.camera, !paused);
    }

    if (IsKeyPressed(KEY_F11)) {
        video_toggle_fullscreen();
    }
    if (IsKeyPressed(KEY_F1)) {
        g->debug = !g->debug;
    }

    if (!paused) {
        // UpdateCamera(&g->player_camera.camera, CAMERA_FIRST_PERSON);
        player_update_camera(&g->player);
        player_update_movement(&g->player, g->debug);
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

    if (g->debug)
        video_draw_debug();

    EndDrawing();
}

