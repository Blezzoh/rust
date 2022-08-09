use iterator_example::Counter;
fn main() {
    let mut counter = Counter::new();

    for _i in 0..6{
        println!("count {:?}", counter.next());
    }
}
