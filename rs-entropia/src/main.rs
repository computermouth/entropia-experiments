use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

const GAME_ORIGIN_POS: f32 = 1000.0;
const CAMERA_STARTY_POS: f32 = 30.0;
const CAMERA_STARTZ_POS: f32 = 30.0;

fn origin_position(x: f32, y: f32, z: f32) -> Vector3 {
    let p = Vector3 {x: x + GAME_ORIGIN_POS, y: y + GAME_ORIGIN_POS, z: z + GAME_ORIGIN_POS};
    if p.x < 0.0 || p.y < 0.0 || p.z < 0.0 {
        eprint!("E: negative space after fake origin {:?}", p)
    }
    p
}

fn game_init() -> (RaylibHandle, RaylibThread) {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, world!")
        .build();
    rl.set_exit_key(None);
    rl.set_target_fps(60);
    (rl, thread)
}

struct Node {
    pos: Vector3,
    size: [f32; 3],
    fcolor: Color,
    wcolor: Color,
}

fn round_init() -> (Camera3D, Vec<Node>) {
    let cam = Camera3D::perspective(
        origin_position(0.0, CAMERA_STARTY_POS, CAMERA_STARTZ_POS),
        origin_position(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );
    let nodes = vec![Node{pos: origin_position(0.0, 0.0, 0.0), size: [1.0,1.0,1.0], fcolor: Color::RED, wcolor: Color::MAROON}];
    
    (cam, nodes)
}

fn main() {

    let (mut rl, thread) = game_init();
    let (mut cam, nodes) = round_init();

    let mut i = 0;
    while !rl.window_should_close() {
        // pro cam???
        rl.update_camera(&mut cam);
        i += 1;
        _ = i;

        let mut draw = rl.begin_drawing(&thread);

        draw.clear_background(Color::RAYWHITE);
        draw.draw_text("GAME", 190, 200, 20, Color::LIGHTGRAY);
        
        let mut draw_3d = draw.begin_mode3D(cam);
        
        for n in &nodes {
            draw_3d.draw_cube(n.pos, n.size[0], n.size[1], n.size[2], n.fcolor);
            draw_3d.draw_cube_wires(n.pos, n.size[0], n.size[1], n.size[2], n.wcolor);
        }
    }
}
