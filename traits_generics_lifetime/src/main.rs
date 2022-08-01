use std::fmt::Display;

// Display is a trait that allows items to be printed to the console
// 'a is the lifetime
// by default, if I remember it is for the one that have the shortest lifespan
// HINT: there are three rules of the rust lifetime
// T is a generic type that has a trait of Display
fn longest_with_announcement<'a, T: Display>(x:&'a str, y:&'a str, ann: T ) -> &'a str{
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

/*
// same function:
fn longest_with_announcement<'a, T>(x:&'a str, y:&'a str, ann: T ) -> &'a str
where T: Display,
{
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}
*/

fn main() {
    let a = String::from("abcdefg");
    let b = String::from("abcdef");

    println!("{}", longest_with_announcement(&a, &b, "test announcement"));
}
