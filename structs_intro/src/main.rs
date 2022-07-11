struct RectangeObject{
    width: usize,
    height: usize,
}

struct RectangleTuple(usize, usize);

/*chose to use references so that objects might not go out of scope */ 
fn area_rect_object(rect: &RectangeObject)->usize {
    rect.width * rect.height
}

fn area_rect_tuple(rect: &RectangleTuple) -> usize{
    rect.1 * rect.0
}

fn main() {
    let rect1 = RectangeObject {
        width: 60,
        height: 30,
    };
    let rect2 = RectangleTuple(45,45);

    /*chose to borrow reference rather than the object going out of scope */ 
    let area_react1 = area_rect_object(&rect1); 
    let area_react2 = area_rect_tuple(&rect2);
    println!("area of rect1 is '{}'. area of rectange 2 is '{}'.", area_react1, area_react2);
}
