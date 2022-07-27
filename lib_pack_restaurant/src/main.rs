
use lib_pack_restaurant::restaurant::hosting;

fn main(){
    let person = "Tim";
    hosting::seat_at_the_table(&person);
    hosting::add_to_waitlist(&person);
    let time = hosting::waiting_time();
    println!("current waiting time {}", &time);
    let service = lib_pack_restaurant::get_services();
    println!("service: {}", &service)
}