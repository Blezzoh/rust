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

   ````println!("{}", "this is som
   ething")```
   - characters are :char
   - has characters
   - tuples:
     eg: `let tup: (i32, f64, u8) = (500, 6.4, 1);`
     printing: `println!(tup);`
     tuples can be destructured: `let (x,y,z) = tup`
     tuples can have multiple types inside
   - arrays: unlike tuples they can only have one type

   ````

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

-
