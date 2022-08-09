pub struct Counter{
    pub count: u32,
}

impl Counter{
    pub fn new() -> Counter{
        Counter {count: 0}
    }
}


// implementing the trait iterator for count
// it needs type and next method
// "type Item" is an associated type of a trait, more in chapter 19
impl  Iterator for Counter {
   type Item = u32; 

   fn next(&mut self) -> Option<Self::Item>{
    if self.count <5{
        self.count +=1;
        Some(self.count)
    }
    else{
        None
    }
   }
}