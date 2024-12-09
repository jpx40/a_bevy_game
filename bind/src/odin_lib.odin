package lib


Vec2   :: struct  #packed{
    x: f32,
    y: f32,
}
@(export)
gen_vec :: proc "c" (x: f32,y:f32) -> Vec2 {

    return Vec2 {x,y}
}