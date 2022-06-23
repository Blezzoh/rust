use std::io;

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
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
    
    
}
