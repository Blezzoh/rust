pub mod restaurant{
    // we want to expost everything
    pub mod hosting;
}


pub fn get_services() -> String {
    String::from("restaurant and bar")
}

// re-exporting 
use restaurant::hosting;
pub fn get_waiting_time() -> String{
    hosting::waiting_time()
}


