use rand::Rng;

pub fn add_to_waitlist(person: &str){
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{}, your wait time will be {} minutes.", person, secret_number);
}

pub fn seat_at_the_table(person: &str){
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("{}, you will be served at the table {}.", person, secret_number);
}

pub fn waiting_time() -> String{
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    secret_number.to_string()
}