// use std::io;

// function no return value
fn fn_try(){
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}

fn fn_returns(x: i32) -> i32 {
    x * x
}
// arrays
fn first_function(){
    let a = [1, 2, 3, 4, 5];

   // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!(
    //     "The value of the element at index {} is: {}",
    //     index, element
    // );
    println!("array: {:?}", a);
}

fn main() {
    let x = 5; // immutable variable
    let mut y = 5; // mutable variable
    println!("immutabable x, mutable y!, {} {}", x, y);
    y=6;
    println!("mutated, {} {}", x, y);

    /**
     constants are immutable, you must annotate the const. More downs
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const, {}", THREE_HOURS_IN_SECONDS);

    /* shadowing uses the lastly defined variables*/
    let x = 12;
    println!("original x, {}", x);
    let x = "something";
    println!("shadow variable x, '{}'",x);

    /*tuple example */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple: {:?}", tup);
    /* arrays */
    first_function();

    /* functions */
    fn_try();
    println!("call to function: {}", fn_returns(12));

}
