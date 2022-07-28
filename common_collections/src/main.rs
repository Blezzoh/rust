fn main() {
    let v = vec![2,3,4,5,6];
    // 2 way to get elements out of vectors
    let first = &v[0];
    let third = v.get(2); // this will come as an option Some(<number>)
    // hard coded 
    let el = match third{
        Some(_num) => 4,
        None => 1000,
    };
    println!("first and third {}, {:?}, {}", first, third, el);
}
