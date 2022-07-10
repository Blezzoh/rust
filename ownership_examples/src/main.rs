fn main() {
    {
        let mut s = String::from("hello");
        s.push_str(", world!");
        println!("{}", s);
    }
    {
        let s = String::from("hello");
        let t =s;
        // println!("{}", s); // this will print an error
        println!("{}",t); // this will work
    }
     {
        let s = String::from("hello");
        let t =s.clone();
        println!("{}", s); // this will work
        println!("{} clone",t); // this will work as well
    }
    {
        // both "s" and "t" are stack variables and are both valid
        let s =5;
        let t = s;
        println!("s is {}, t is {}", s,t);
    }
    {
        // tuples of stack variables
        let tup1: (i32, f64, u8) = (500, 6.4, 1);
        let tup2 = tup1;
        println!("tup1 is {:?}, tup2 is {:?}", tup1,tup2);
 
    }
    {
        let s = String::from("hello");
        takes_ownership(s); /* the function moves the variable, and the variable is out of scope after this call*/
        println!("if we tried to print s here we will fail");
        let t = 5;
        makes_a_copy(t); /* the variable is copied by the function, and it will still be in scope after the function call*/
        println!("still in scope {}", t)
    }
    {
        let s2 = String::from("hello"); // s2 goes into scope
        let (s3, len) = str_len(s2); /* s2 goes out scope, then return values of function transferred to new variables */

        println!("string '{}' of length {}", s3, len);

    }
    {
        // borrowing ownership by passing in a reference
        let s1 = String::from("hello");
        let len = borrows_ownership(&s1);
        println!("string '{}' of length {}", s1, len);

    }
    {
         let mut s1 = String::from("hello");
         println!("original '{}'", s1);
         borrows_and_mutates(&mut s1);
         println!("mutated reference '{}'", s1);
    }
    {
        // string slice
        let s = String::from("hello world");
        let part1 = &s[0..5];
        let part2 = &s[6..11];
        println!("{} {}", part1, part2);
    }
    {
        let mut s = String::from("hello world");
        let word = get_first_world(&s); // 1 borrow of &s as immutable
        // s.clear(); // 2nd borrow of &s as mutable  will cause
        println!("first word is {}", word);
        /* This is bad code because we don't need to use mutable "s" but it is for the sake of an example */
    }
    {
        let a =[1,2,3,4,5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2,3]);
    }

}


fn makes_a_copy(x:isize){
    println!("copied {}", x);
}

fn takes_ownership(x:String){
    println!("owns {}",x);
}

fn str_len(x:String)->(String, usize){
  let size = x.len();
  
  (x, size)
}

/* referencing and borrowing */
fn borrows_ownership(x: &String) -> usize{
  x.len()
}

fn borrows_and_mutates(x: &mut String){
  x.push_str(", world");
}

fn get_first_world(s: &str) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

