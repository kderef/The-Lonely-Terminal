#include <stdio.h>

#include "game.h"

int main(void) {
    const GameConfig conf = {
            .window_title = "Hello",
            .screen_w = 1280,
            .screen_h= 720,
            .target_fps = 170,
            .window_resizable = false,
            .vsync = false,
            .msaa_4x = true,
    };
        
    Game g = game_init(conf);

    game_open(&g);
    game_load(&g);

    while (g.running) {
        game_update(&g);
        game_draw(&g);
    }

    game_close(&g);

    return 0;
}
