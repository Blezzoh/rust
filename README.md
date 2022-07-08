# rust

crash course

## comments

#### I. hello world

1. helloworld: "!" on printfn means it is a macro otherwise if it was a function it would have been be "printfn"
2. hello-cargo: "cargo new" creates a new cargo project, "cargo build" build project, "cargo run" builds and also runs the project, and "cargo check" build the project without generating binaries for error checking
3. comments are the same as java comments

#### II. Guessing game

1. imports "use std::io" in java this would be "import io from std".
   Same way on line 16; we could have made this easier by typing importing stdin "use std::io::stdin"
2. mutable variables: by default variables are immutable in rust, so we used "let mut guess" instead of "let guess" to use a mutable variable
3. uses the first crate dependency "rand". "rand::Rng" used because of traits that will be defined in section 10 of https://doc.rust-lang.org/book/.
4. "cmp" compares variables at the left and right and returns "Ordering::<Greater/Lesser/Equal>"
5. "match" expression is logically like a squitch statement.
6. the code runs in an infinite loop until match is found.

#### III. Concepts

1. Variable and mutability

   - let <varname>, let mut <varname>: mutable vs immutable
   - const are immutable variables, they must also be annotated
   - shadowing is declaring two variables that have the same name, the lastly defined variable is the one that is used. shadowing is different to mut because we can use even a different datatype.

2. Data types:

   - rust is statically typed, which means that it must know the type of each variable at compile time.
   - integer types:
     |Length| Signed| Unsigned|
     |---|---|---|
     |8-bit| i8 |u8|
     |16-bit| i16 |u16|
     |32-bit| i32| u32|
     |64-bit |i64 |u64|
     |128-bit |i128 |u128|
     |arch |isize| usize|
   - floats:
     integer types:
     |Length| Signed|
     |---|---|
     |8-bit| f8 |
     |16-bit| f16 |
     |32-bit| f32|
     |64-bit |f64 |
     |128-bit |f128 |
     |arch |fsize|
     "size" will use the maximum available capacity.
   - you cannot mix floats and integers. eg: `12.0 +1`. This will generate a compilation error
   - booleans go by :bool
   - literal string can be used on multiple lines. eg:

   ```
   println!("{}", "this is som
   ething")
   ```

   - characters are :char
   - has characters
   - tuples:

     - eg: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
     - printing: `println!("{:?}",tup);`
     - tuples can be destructured: `let (x,y,z) = tup`
     - tuples can have multiple types inside

   - arrays: unlike tuples they can only have one type

3. Functions:
   - functions start with "fn" and the arguments are given a type by adding the ":<type>".
   - expressions:
     - expressions can be evaluated to a value; so a function is an expression. A macro is as well.
     - statements such as "let x = 6;" do not.
     - expression at the end of a function do not have a semi-colon
   - functions that return a value have this form:
     ```
      fn <name> (arg: <type>, ...) -> <type> {
      ...
      <expression>
      }
     ```
4. Comments

- `//this is a comment in rust`
- `/* This is also a comment */`

5. control flow:

- this is an example of an if block:

```
 if x<0 {
        println!("x is a negative number");
    }
    else if x==0{
        println!("x is equal to zero");
    }
    else{
        println!("x is a positive number");
    }
```

- you can assign a variable on condition: `let x = if condition {value} else {another_value};`
- infinite loops uses a statement loop:
  `loop{...}`

- loops can return a value by using an expression at the end .

eg:

```
let mut count = 0;
let value = loop{
  count += 1;
  if count ==10 {
    break count;
  }
}
println!("{}", count);
```

- while loops have this format `while condition {...}`
- you can iterate through a collection using "for in". `for element in collection{}`
- you can iterate n times using a range like listing. `for num in start..end {}`.

#### IV. Understating Ownership

##### IV. I. What is Ownership

1. Intro:

- Rust has unique features that can guarrantee users memory safety without needing a garbage collector.
- this is why it is important to understand how ownership works

2. Definitions:

- Every language has a way to manage memory whether by using a garbage collection or allocating memory.
  Rust uses a feature that defines memory ownership and if violated the program won't compile. It also doesn't
  slow down the program.
- Rust is a systems language and having a value of the heap or stack affect how the language behaves, so that's why you have to make certain decisions.
- Parts of ownership are defined in relation to the stack or the heap.
- Stack is a last in, first out. Values pushed to the stack must have a known, and fixed size.
- Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
- The heap is less organized and when you put data here, the memory allocator finds an empty spot big enough, marks it as being used, and then returns a pointer.
- This process is called "allocating on the heap" or just "allocating". Pushing values on the stack is not called "allocating".
- The pointer to the heap has a known and fixes size. It can then been stored to the stack and be found by using this pointer on the stack.
- Allocating on the heap takes relatively much more time than on pushing to the stack since the memory allocator has to search which it never does while using the stack.
- Accessing data in the heap takes again longer because you have to follow a point on the stack. Memory can also do many jumps since it may follow multiple locations instead of just one for accessing the stack.

- when your code call a function, local variables and functions arguments(including pointers to the heap if used), are all pushed to the stack. when a function call finishes those values are popped off.
- `Keeping track of what parts of code are using which data on the heap, minimizing the amount of duplicate data on the heap, and cleaning unused data so that you don't run out of space are all problems that ownership addresses.`

3. Ownership Rules:

- Each value in Rust has an owner
- There can only be one owner at a time
- when the owner goes out of scope, the value will be dropped.

4. Variable Scope:

- Here we will focus on scope material that might different from other languages.
- The string type is a good example of explaining how values are stored on the heap.
- We will use "String" type instead of literals because String can be mutated but literals cannot.

```
{
  // start of scope
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
} //end of scope
```

- We know the contents at compile time. This is why they are fast and efficient. This property comes from literals immutability.
- When we use the mutable String, the size is unkown at compile time since it can change size. We have to request the memory allocator at run time.
- what happens here is that whenever "s" in our example goes out of scope, it is dropped by a special function called "drop". We don't use a garbage collector or memory allocation strategies.
- This example is simple but it can get complicated when some other parts of the code are using the same variable.

```
let s = String::from("hello");
let t = s;
```

- A string is made of 3 parts under the hood:

```
1. a pointer to the address it is stored at in heap.
2. length
3. capacity
```

- when we assign "t", a copy of of the pointer, length, and capacity is made but the value is not created again in the heap.
- if Rust calls drop, there will be a double free error since rust will try to drop "s" and "t" at the same time which can also lead to security vulnerabilities in the code base.
- In order to ensures that this doesn't happen, "s" becomes invalid after "t" is created.

```
  let s = String::from("hello");
  let t =s;
  // println!("{}", s); // this will print an error
  // println!("{}",t); // this will work
```

- This process of invalidating "s" is called a move(works like a shallow copy but also invalidate the previous value).

- if we wanted to keep both, we have to make a clone.

```
  let s = String::from("hello");
  let t =s.clone();
  // println!("{}", s); // this will work
  // println!("{}",t); // this will work as well
```

- Rust doesn't do an automatic deep copy, it must be called by the user. It is relatively more expensive than a "move".

5. Stack-Only data

- literalls that have a known fixed size are stored on the stack and they don't make a move like for example the "String" variables above.

```
// both "s" and "t" are stack variables and are both valid
let s =5;
let t = s;
```

- No need to make a clone in the example above if we wanted to keep both variables. They implement a copy of their own.

- Types that that don't need a clone: `All integers types, Boolean type, floats, character type, and tuples tof types that have their own Copy.`

6. Ownership and Functions

- ownership while passing a value to a function follow the same rules of copying or moving. When a variable is a stack variable it is moved, and when a variable is heap variable, it is copied.

```
{
  let s = String::from("hello");
  takes_ownership(s); /* the function moves the variable, and the variable is out of scope after this call*/
  let t = 5;
  makes_a_copy(t); /* the variable is copied by the function, and it will still be in scope after the function call*/


}
```

7. Return Values and Ownership

- Returning values can also transfer ownership. Next example the return value ownership into s1.

```
fn gives_ownership(){
  let s = String::from("hello");
  s // return value
}

fn main(){
  let s1 = gives_ownership();
}

```

- Next example shows 2 movement of ownership

```
fn main(){
  let s2 = String::from("hello"); // s2 comes in scope
  let s3 = takes_and_gives_ownership(s2); /* s2 goes out of scope by moving it to the function, and function return moves the ownership to s3.
}

fn takes_and_gives_ownership(x:String){
  x
}
```

- functions can have multiple return values in form of tuple where users can move ownership to their own variables.

```
fn takes_and_gives_ownership(x:String){
  (x, s.len())
}
```
