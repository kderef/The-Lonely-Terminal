#pragma once

typedef struct {
    
} Audio;

Audio audio_new();

void audio_init(Audio*);
void audio_close(Audio*);
