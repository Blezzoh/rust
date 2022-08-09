use add_one;
use substract_one;
fn main() {

    let x = 123;
    println!("sub one {} , add one {}", add_one::add_one(x), substract_one::sub_one(x));
}
