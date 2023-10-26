
#include <stdio.h>

#include "raybridge.h"

#include "raylib/src/raylib.h"

#define WINDOW_WIDTH         640
#define WINDOW_HEIGHT        480
#define GAME_ORIGIN_POS   1000.0f
#define CAMERA_STARTY_POS   30.0f
#define CAMERA_STARTZ_POS   30.0f
#define CAMERA_TARGET_POS    0.0f

Vector3 origin_position(float x, float y, float z) {
    Vector3 p = {.x = x + GAME_ORIGIN_POS, .y = y + GAME_ORIGIN_POS, .z = z + GAME_ORIGIN_POS };
    if (p.x < 0 || p.y < 0 || p.z < 0) {
        fprintf(stderr, "E: negative space after fake origin: {%f,%f,%f}\n", p.x, p.y, p.z);
    }
    return p;
}

void game_init(int width, int height, const char * name){
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(width, height, name);
    SetExitKey(KEY_NULL);
    SetTargetFPS(60);
}

Camera3D camera;

void round_init(float x, float y, float z, float fov){
    camera.position = origin_position(x, y, z);
    camera.target = origin_position(0, 0, 0);
    camera.up = (Vector3){ 0, 1, 0 };
    camera.fovy = fov;
    camera.projection = CAMERA_PERSPECTIVE;
}

void update_camera(rb_vec3 mov, rb_vec3 rot, float zoom){
    Vector3 vmov = {.x = mov.x, .y = mov.y, .z = mov.z };
    Vector3 vrot = {.x = rot.x, .y = rot.y, .z = rot.z };
    UpdateCameraPro(&camera, vmov, vrot, zoom);
}

void begin_drawing(){
    BeginDrawing();
    
    ClearBackground(RAYWHITE);
    DrawText("GAME", 190, 200, 20, LIGHTGRAY);
    
    BeginMode3D(camera);
}

void draw_node(rb_node n){
    Vector3 pos = origin_position(n.pos[0], n.pos[1], n.pos[2]);
    Color fcolor = {.r = n.fcolor[0], .g = n.fcolor[1], .b = n.fcolor[2], .a = n.fcolor[3]};
    Color wcolor = {.r = n.wcolor[0], .g = n.wcolor[1], .b = n.wcolor[2], .a = n.wcolor[3]};

    DrawCube(pos, n.size[0], n.size[1], n.size[2], fcolor);
    DrawCubeWires(pos, n.size[0], n.size[1], n.size[2], wcolor);
}

void end_drawing(){
    EndMode3D();
    EndDrawing();
}

int window_should_close(){
    return WindowShouldClose();
}