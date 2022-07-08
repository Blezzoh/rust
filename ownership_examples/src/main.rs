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
