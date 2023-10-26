
#include <stdio.h>

#include "raylib/src/raylib.h"
#include "libdsa/vector.h"

#define WINDOW_WIDTH         640
#define WINDOW_HEIGHT        480
#define GAME_ORIGIN_POS   1000.0f
#define CAMERA_STARTY_POS   30.0f
#define CAMERA_STARTZ_POS   30.0f
#define CAMERA_TARGET_POS    0.0f

typedef struct {
    Vector3 pos;
    float size[3];
    Color fcolor;
    Color wcolor;
} Node_t;

Vector3 origin_position(float x, float y, float z) {
    Vector3 p = {.x = x + GAME_ORIGIN_POS, .y = y + GAME_ORIGIN_POS, .z = z + GAME_ORIGIN_POS };
    if (p.x < 0 || p.y < 0 || p.z < 0) {
        fprintf(stderr, "E: negative space after fake origin: {%f,%f,%f}\n", p.x, p.y, p.z);
    }
    return p;
}

void game_init(){
    SetConfigFlags(FLAG_WINDOW_RESIZABLE);
    InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Entropia");
    SetExitKey(KEY_NULL);
    SetTargetFPS(60);
}

Camera3D camera;
vector * nodes = NULL;

void round_init(){
    camera.position = origin_position(0, CAMERA_STARTY_POS, CAMERA_STARTZ_POS);
    camera.target = origin_position(0, 0, 0);
    camera.up = (Vector3){ 0, 1, 0 };
    camera.fovy = 45.0f;
    camera.projection = CAMERA_PERSPECTIVE;
    
    nodes = vector_init(sizeof(Node_t));
    vector_push(nodes, &(Node_t){.pos = origin_position(0,0,0), .size = {1,1,1}, .fcolor = RED, .wcolor = MAROON});
}

int main(void)
{
    game_init();
    round_init();
    
    for(int i = 0; !WindowShouldClose(); i++){
        UpdateCameraPro(&camera, (Vector3){0}, (Vector3){0}, (float)i * -.01f);
        i++;
        
        BeginDrawing();
        
        ClearBackground(RAYWHITE);
        DrawText("GAME", 190, 200, 20, LIGHTGRAY);
        
        BeginMode3D(camera);
        
        size_t nlen = vector_size(nodes);
        Node_t * nodearr = vector_begin(nodes);
        for(size_t i = 0; i < nlen; i++){
            Node_t n = nodearr[i];
            DrawCube(n.pos, n.size[0], n.size[1], n.size[2], n.fcolor);
            DrawCubeWires(n.pos, n.size[0], n.size[1], n.size[2], n.wcolor);
        }
        
        EndMode3D();
        EndDrawing();
    }
    
    CloseWindow();

    return 0;
}