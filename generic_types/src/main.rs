
// generic comparison function. Uses PartialOrd trait to indicate order in a list and copy to indicate primitive 
fn get_largest<T: PartialOrd + Copy>(collection_list:Vec<T>) -> T{
    let mut largest = collection_list[0];
    for item in collection_list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main() {
    let l1 = vec![34,13,543,13134,36,563,2534,5465,153341];
    let l2 = vec!['q','s','x','d','r','g','b'];
    let large1 = get_largest(l1);
    let large2 = get_largest(l2);
    println!("largest in number is {} and in characters is '{}'!", large1, large2);
}
