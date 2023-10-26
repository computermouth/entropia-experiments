
mod internal;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Node {
    pub pos: Vector3,
    pub size: Vector3,
    pub fcolor: Color,
    pub wcolor: Color,
}

pub fn game_init(width: i32, height: i32, name: &str){
    unsafe {
        let c_str = std::ffi::CString::new(name).expect("failed to convert string");
        internal::game_init(width, height, c_str.as_ptr());
    }
}

pub fn round_init(x: f32, y: f32, z: f32, fov: f32){
    unsafe {
        internal::round_init(x, y, z, fov);
    }
}

pub fn update_camera(mov: Vector3, rot: Vector3, zoom: f32){

    let rb_mov = internal::rb_vec3 { x: mov.x, y: mov.y, z: mov.z };
    let rb_rot = internal::rb_vec3 { x: rot.x, y: rot.y, z: rot.z };

    unsafe {
        internal::update_camera(rb_mov, rb_rot, zoom);
    }
}

pub fn begin_drawing(){
    unsafe {internal::begin_drawing();}
}

pub fn draw_node(n: Node){
    
    let rbn = internal::rb_node{
        pos: [n.pos.x, n.pos.y, n.pos.z],
        size: [n.size.x, n.size.y, n.size.z],
        fcolor: [n.fcolor.r, n.fcolor.g, n.fcolor.b, n.fcolor.a],
        wcolor: [n.wcolor.r, n.wcolor.g, n.wcolor.b, n.wcolor.a],
    };
    
    unsafe {internal::draw_node(rbn);}
}

pub fn end_drawing(){
    unsafe {internal::end_drawing();}
}

pub fn window_should_close() -> bool {
    unsafe {
        match internal::window_should_close() {
            0 => return false,
            _ => return true,
        }
    }
}