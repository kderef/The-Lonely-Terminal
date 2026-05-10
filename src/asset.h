#pragma once

#include "raylib.h"

typedef enum {
    AID_NONE = 0,
    AID_SKY1,


    AID_MAX
} AssetID;

typedef enum {
    ASSET_SOUND,
    ASSET_TEXTURE,
} AssetType;


typedef struct {
    AssetType type;
    AssetID id;
    bool loaded;

    union {
        Sound sound;
        Texture texture;
    };
} Asset;

Asset* asset_load(AssetID);
void asset_unload(AssetID);
void asset_unload_all();

Asset* asset_get(AssetID);
const char* asset_path(AssetID);
