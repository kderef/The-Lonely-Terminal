#include "audio.h"

#include <raylib.h>

Audio audio_new() {
    
    return (Audio) {
    
    };
}

void audio_init(Audio* a) {
    InitAudioDevice();
}

void audio_close(Audio* a) {
    CloseAudioDevice();
}
