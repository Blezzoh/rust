fn multiple_conditions(x: isize){
    if x<0 {
        println!("x is a negative number");
    }
    else if x==0{
        println!("x is equal to zero");
    }
    else{
        println!("x is a positive number");
    }
}

fn value_on_condition(value: isize){
    let x = if value<0 {"negative number"} else {"not a negative number"};
    println!("{}",x);
}
fn main() {
    let x :isize = -100;
    multiple_conditions(x);
    value_on_condition(x);
    let mut count = 0;
    let collection = [10,12,15,30];
    // loop that returns a value
    let value = loop {
        count += 1;
        if count ==10 {
            break count;
        }
    };
    println!("{}", value);

    // for in loop for a collection
    for val in collection{
        println!("element of a collection {}", val);
    }
    // for loop in a range like statement
    for num in 1..5{
        println!("printing through a range {}", num);
    }
}

