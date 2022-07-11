struct RectangeObject{
    width: usize,
    height: usize,
}

impl RectangeObject{
    // first two are methods bc they have &self as the first parameter
    fn area (&self) -> usize{
        self.width * self.height
    }
    fn can_hold(&self, other: &RectangeObject)-> bool{
        self.width > other.width && self.height > other.height
    }
    // assosciated function
    fn square(size:usize)->RectangeObject{
        RectangeObject{
            width: size,
            height:size,
        }
    }
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

    /* Using methods */
    let rect3 = RectangeObject{
        width: 55,
        height: 31,
    };
    let rect4 = RectangeObject::square(29); // using assosciated function
    println!("area of rect 1 using method is '{}'", rect1.area());
    println!("area of rect 3 using method is '{}'", rect3.area());
    println!("area of rect 4(square) using method is '{}'", rect4.area());

    println!("can rect1 hold react3 '{}' ", rect1.can_hold(&rect3));
    println!("can rect1 hold react4 '{}' ", rect1.can_hold(&rect4));
}
