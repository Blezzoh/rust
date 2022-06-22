# rust

crash course

## comments

#### I hello world

1. helloworld: "!" on printfn means it is a macro otherwise if it was a function it would have been be "printfn"
2. hello-cargo: "cargo new" creates a new cargo project, "cargo build" build project, "cargo run" builds and also runs the project, and "cargo check" build the project without generating binaries for error checking
3. comments are the same as java comments

#### II Guessing game

1. imports "use std::io" in java this would be "import io from std".
   Same way on line 16; we could have made this easier by typing importing stdin "use std::io::stdin"
2. mutable variables: by default variables are immutable in rust, so we used "let mut guess" instead of "let guess" to use a mutable variable
3. uses the first crate dependency "rand". "rand::Rng" used because of traits that will be defined in section 10 of https://doc.rust-lang.org/book/.
4. "cmp" compares variables at the left and right and returns "Ordering::<Greater/Lesser/Equal>"
5. "match" expression is logically like a squitch statement.
6. the code runs in an infinite loop until match is found.
