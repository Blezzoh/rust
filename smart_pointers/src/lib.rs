use std::ops::Deref;
use std::rc::Rc;

// it's like a linked list
// explains Box Smart pointer
pub enum List{
    Cons(isize, Box<List>),
    // we use the Box smart pointer because we don't know exactly how big this recursive data structure can be
    Nil
}

pub struct MyBox<T>(T);


impl<T> MyBox<T>{
    pub fn new(x: T) ->MyBox<T>{
        MyBox(x)
    } 
}

// implementing this trait allows us to have a pointer(thus smart pointer)
// this is how it works, when you do *(<instance of my box>), rust will automatically do *(<instance of my box>.deref())
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// --- this part is dedicated to reference counting smart pointers(s_p). 
// reference counting s_p allows sharing of ownership of the data



pub enum ListRc {
    Cons(isize, Rc<ListRc>),
    Nil
}
// in order to use List we well have to import self::List::{Cons, Nil}
//use self::List::*;
use self::ListRc::*;

/*
the idea of the function

y---
    |->x
z---

Both y and z share ownership of x.

Note that sharing ownership doesn't change the fact that the data they point to is not mutable since Rc only allows immutable borrows. 
If they were, we will be breaking ownership rules.


*/


pub fn share_ownership()-> ListRc{
    /* this code commented will generate errors because of the borrow checker. x is moved by the time we create y.
        this is where Rc comes into play */
    //let x = Cons(1, Box::new(Cons(2, Box::new(Cons(2, Box::new(Nil))))));
    //let y = Cons(6, Box::new(x));
    //let z = Cons(7, Box::new(x));

    /* using reference counter */
    let x = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(2, Rc::new(Nil)))))));
    println!("refernce count of &x after creation {}", Rc::strong_count(&x));
    // calling the clone function doesn't work like a regular clone instead increases the counter of ownership
    let y = Cons(6, Rc::clone(&x));
    println!("refernce count of &x after creation of x {}", Rc::strong_count(&x));

    {
        let z = Cons(7, Rc::clone(&x));
        println!("refernce count of &x after creation of z {}", Rc::strong_count(&x));
    }
    println!("refernce count of &x after z goes out of scope {}", Rc::strong_count(&x));

    y
}