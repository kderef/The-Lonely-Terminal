#ifndef GAME_H
#define GAME_H

#include <raylib.h>
#include <stdbool.h>

#include "skybox.h"
#include "menu.h"
#include "player.h"

/******************************************************************************/
// game


typedef struct {
    const char* window_title;
    int screen_w;
    int screen_h;

    int target_fps;

    bool vsync;
    bool msaa_4x;
    bool fullscreen;
    bool window_resizable;
} GameConfig;

typedef struct {
    GameConfig config;
    bool running;
    bool debug;


    // UI
    PauseMenu pause_menu;
        
    // camera system
    Player player;
    Skybox skybox;
} Game;

Game game_init(GameConfig);
void game_load(Game*);

void game_open(Game*);
void game_close(Game*);

// loop

void game_update(Game*);
void game_draw(Game*);



#endif // GAME_H
