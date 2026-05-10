#include "skybox.h"

#include "raylib.h"
#include "rlgl.h"

Skybox skybox_generate(Texture tex) {
    Skybox s;

    s.cube = GenMeshCube(1.0, 1.0, 1.0);
    s.model = LoadModelFromMesh(s.cube);

    bool use_hdr = false;

    // NOTE: web GLSL version is 100 not 330
    s.model.materials[0].shader = LoadShader(
        "shaders/skybox.vs",
        "shaders/skybox.fs"
    );

    SetShaderValue(s.model.materials[0].shader, GetShaderLocation(s.model.materials[0].shader, "environmentMap"), (int[1]){ MATERIAL_MAP_CUBEMAP }, SHADER_UNIFORM_INT);
    SetShaderValue(s.model.materials[0].shader, GetShaderLocation(s.model.materials[0].shader, "doGamma"), (int[1]){ use_hdr? 1 : 0 }, SHADER_UNIFORM_INT);
    SetShaderValue(s.model.materials[0].shader, GetShaderLocation(s.model.materials[0].shader, "vflipped"), (int[1]){ use_hdr? 1 : 0 }, SHADER_UNIFORM_INT);
    
    // cubemap
    
    Shader cubemap_shader = LoadShader(
        "shaders/cubemap.vs",
        "shaders/cubemap.fs"
    );

    SetShaderValue(cubemap_shader, GetShaderLocation(cubemap_shader, "equirectangularMap"), (int[1]){ 0 }, SHADER_UNIFORM_INT);

    Image img = LoadImageFromTexture(tex);

    // apply image
    s.model.materials[0].maps[MATERIAL_MAP_CUBEMAP].texture = LoadTextureCubemap(img, CUBEMAP_LAYOUT_AUTO_DETECT);
    UnloadImage(img);

    return s;
}

void skybox_unload(Skybox* sb) {
    UnloadModel(sb->model);
    // UnloadMesh(sb->cube);
}

void skybox_draw(Skybox* sb) {
    rlDisableBackfaceCulling();
    rlDisableDepthMask();

    DrawModel(sb->model, (Vector3){0, 0, 0}, 1.0f, WHITE);

    rlEnableBackfaceCulling();
    rlEnableDepthMask();
}
