use glam::Vec2;

// extern "C" {
//     pub fn gen_vec(x:f32,y:f32) -> Vec2;

// }

use lazy_static::lazy_static;
use libloading::Library;

// fn load_lib() -> Library {

//     unsafe { libloading::Library::new("./bind/odin_lib.so").expect("failed to link lib") }
// }
// lazy_static! {

//   pub static ref   LIB: Libary = load_lib();

// }

// pub fn get_vec2(x: f32, y: f32) -> Vec2 {
//     unsafe {
//         let lib = libloading::Library::new("./bind/odin_lib.so").expect("failed to link lib");
//         let func: libloading::Symbol<unsafe extern "C" fn(x: f32, y: f32) -> Vec2> =
//             lib.get(b"gen_vec").expect("failed to call function");
//         func(x, y)
//     }
// }
