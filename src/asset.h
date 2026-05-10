#pragma once

#include "raylib.h"

#define ASSETS \
    X(SKY1, ASSET_TEXTURE, "textures/skybox_test1.png")


#define X(NAME, ...) AID_##NAME,
typedef enum {
    AID_NONE = 0,
    
    ASSETS

    AID_MAX
} AssetID;
#undef X

typedef enum {
    ASSET_SOUND,
    ASSET_TEXTURE,
} AssetType;

#define X(NAME, TYPE, PATH) [AID_##NAME] = {PATH, TYPE},
static const struct {const char* path; AssetType type; } ASSET_INFO[AID_MAX] = {
    [AID_NONE] = {"NONE", 0},
    ASSETS
};
#undef X

typedef struct {
    AssetType type;
    AssetID id;
    bool loaded;

    union {
        Sound sound;
        Texture texture;
    };
} Asset;

// Load the asset into the global asset pool, and return a reference to it.
Asset* asset_load(AssetID);

// Unload the asset from the global asset pool
void asset_unload(AssetID);

// Unload all the assets from the global asset pool
void asset_unload_all();

// Returns a pointer to the asset if it was loaded, otherwise NULL
Asset* asset_get(AssetID);

// Return path associated with AssetID, will be NULL on an invalid asset id
const char* asset_path(AssetID);
