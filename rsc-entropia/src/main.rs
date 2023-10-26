use raybridge::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

const CAMERA_STARTY_POS: f32 = 30.0;
const CAMERA_STARTZ_POS: f32 = 30.0;

fn main() {

    raybridge::game_init(WINDOW_WIDTH, WINDOW_HEIGHT, "GAME");
    raybridge::round_init(0.0, CAMERA_STARTY_POS, CAMERA_STARTZ_POS, 45.0);
    
    while !raybridge::window_should_close() {
        
        // update_camera();
        
        raybridge::begin_drawing();
        
        raybridge::draw_node(raybridge::Node{
            pos: Vector3 {x: 0.0, y: 0.0, z: 0.0},
            size: Vector3 {x: 1.0, y: 1.0, z: 1.0},
            fcolor: Color {r: 230, g: 41, b: 55, a: 255},
            wcolor: Color {r: 190, g: 33, b: 55, a: 255},
        });
        
        raybridge::end_drawing();
    }
    
}