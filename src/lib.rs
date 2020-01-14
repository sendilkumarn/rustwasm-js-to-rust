use wasm_bindgen::prelude::*;

//1
#[wasm_bindgen(module = "./point")]
extern "C" {
    //2
    pub type Point;

    //3
    #[wasm_bindgen(constructor)]
    fn new(x: i32, y: i32) -> Point;

    //4
    #[wasm_bindgen(method, getter)]
    fn get_x(this: &Point) -> i32;

    #[wasm_bindgen(method, getter)]
    fn get_y(this: &Point) -> i32;

    //5
    #[wasm_bindgen(method, setter)]
    fn set_x(this: &Point, x:i32) -> i32;

    #[wasm_bindgen(method, setter)]
    fn set_y(this: &Point, y:i32) -> i32;

    // 6
    #[wasm_bindgen(method)]
    fn add(this: &Point, p: Point);
}

#[wasm_bindgen]
//7
pub fn get_precious_point() -> Point {
    let p = Point::new(10, 10);
    let p1 = Point::new(3, 3);
    // 8
    p.add(p1);
    p
}
