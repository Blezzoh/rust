

use smart_pointers::MyBox;
// use smart_pointers::List::{Cons, Nil};
use smart_pointers::share_ownership;

fn print_int(x:&i32){
    println!("integer passed , {}", x );
}
fn main() {
    // not useful
    let b = Box::new(5);
    println!("b values is {}", b);

    // case where using Box pointers is necessary since it is hard to know the size before compilation
    // commented out is how we will instantiatiate
    // let x = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));
    
    // example of Deref trait
    let z = MyBox::new(5);

    println!("dereferecing works: address[0] of z has {}", *z);
    print_int(&z); // this is how it works, &MyBox<i32> -> &i32 since rust has automatic  deref coersion. (kinda looks like casting)
    // if this was a box string that we wanted to print (aka print_str(x: &str){}, it will be &MyBox<String -> &String -> &strr


    // reference counting
    let x = share_ownership();


    // Refcell that allows user to mutate a reference
    // if you want to read more about interior mutability read chapter 15.5 of The Book

    let y = std::cell::RefCell::new(123);
    *y.borrow_mut() +=123;
    println!("mutate value of y is = {}", y.borrow());
}
