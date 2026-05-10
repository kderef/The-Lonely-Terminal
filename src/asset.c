#include <stdlib.h>

#include "asset.h"
#include "raylib.h"

#define AID_INVALID(AID) ((AID) <= AID_NONE || (AID) >= AID_MAX)

Asset g_assets[AID_MAX] = {0};

const char* asset_path(AssetID id) {
    if (AID_INVALID(id)) return NULL;
    return ASSET_INFO[id].path;
}

Asset* asset_get(AssetID id) {
    if (AID_INVALID(id)) return NULL;
    if (!g_assets[id].loaded) return NULL;

    return &g_assets[id];
}

Asset* asset_load(AssetID id) {
    if (g_assets[id].loaded) return &g_assets[id];

    const char* path = ASSET_INFO[id].path;

    Asset* a = &g_assets[id];

    a->type = ASSET_INFO[id].type;
    a->id = id;
    a->loaded = true; // TODO: fix loaded true

    switch (g_assets[id].type) {
        case ASSET_SOUND:
            a->sound = LoadSound(path);
            break;
        case ASSET_TEXTURE:
            a->texture = LoadTexture(path);
            break;
    }

    return a;
}

void asset_unload(AssetID id) {
    if (!g_assets[id].loaded) return;
    if (id <= 0 || id > AID_MAX) return;

    Asset* a = &g_assets[id];
    
    switch (a->type) {
        case ASSET_SOUND:
            UnloadSound(a->sound);
            break;
        case ASSET_TEXTURE:
            UnloadTexture(a->texture);
            break;
    }

    a->loaded = false;
}

void asset_unload_all() {
    for (AssetID aid = AID_NONE + 1; aid < AID_MAX; aid++) {
        asset_unload(aid);
    }
}
