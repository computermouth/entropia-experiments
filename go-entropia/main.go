package main

import (
	"fmt"
	"image/color"

	rl "github.com/gen2brain/raylib-go/raylib"
)

const (
	WINDOW_WIDTH = 640
	WINDOW_HEIGHT = 480
	GAME_ORIGIN_POS   = 1000.0
	CAMERA_STARTY_POS = 30.0
	CAMERA_STARTZ_POS = 30.0
	CAMERA_TARGET_POS = 0.0
)

func origin_position(x, y, z float32) rl.Vector3 {
	p := rl.Vector3{X: x + GAME_ORIGIN_POS, Y: y + GAME_ORIGIN_POS, Z: z + GAME_ORIGIN_POS}

	if p.X < 0 || p.Y < 0 || p.Z < 0 {
		fmt.Println("E: negative space after fake origin ", p)
	}

	return p
}

func game_init() {
	rl.SetConfigFlags(rl.FlagWindowResizable)
	rl.InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, "Entropia")
	rl.SetExitKey(rl.KeyNull)
	rl.SetTargetFPS(60)
}

type Node struct {
	pos    rl.Vector3
	size   [3]float32
	fcolor color.RGBA
	wcolor color.RGBA
}

var Camera rl.Camera3D
var Nodes []Node

func round_init() {
	Camera.Position = origin_position(0, CAMERA_STARTY_POS, CAMERA_STARTZ_POS)
	Camera.Target = origin_position(0, 0, 0)
	Camera.Up = rl.Vector3{X: 0, Y: 1, Z: 0}
	Camera.Fovy = 45
	Camera.Projection = rl.CameraPerspective

	Nodes = append(Nodes, Node{pos: origin_position(0,0,0), size: [3]float32{1,1,1}, fcolor: rl.Red, wcolor: rl.Maroon})
}

func main() {
	game_init()
	defer rl.CloseWindow()

	round_init()

	i := 0

	for !rl.WindowShouldClose() {
		rl.UpdateCameraPro(&Camera, rl.Vector3{}, rl.Vector3{}, float32(i)*-.01)
		i++

		rl.BeginDrawing()

		rl.ClearBackground(rl.RayWhite)
		rl.DrawText("GAME", 190, 200, 20, rl.LightGray)

		rl.BeginMode3D(Camera)
		
		for _, v := range Nodes {
			rl.DrawCube(v.pos, v.size[0], v.size[1], v.size[2], v.fcolor)
			rl.DrawCubeWires(v.pos, v.size[0], v.size[1], v.size[2], v.wcolor)
		}

		rl.DrawGrid(10, 1.0)
		rl.EndMode3D()

		rl.EndDrawing()
	}
}
