#### V. Structs

Structures are a group of elements that contains a group of dta closely related. They are like objects data in object-oriented programming.

##### 5.1 Definining and Initating a structures

- structures are definined using the world struct much like c:

```
struct TwoTeamsGame{
    stadium: String,
    team1: String,
    team2: String,
    score1: usize,
    score2: usize,
};
```

- you can build struct to be mutable or not using mut:

```
let mut user = User{
    username: String::from("user1"),
    email: String::from("user@user.com"),
};
user.name = String::from("user2")
// or
let user = User{
    username: String::from("user1"),
    email: String::from("user@user.com"),
};
```

- you can user a function to initiate a struct

```
fn build_user(username::String, email::String) -> User{
    User {
        username: username,
        email: email,
    }
}
```

- You can use a lazy like notation while initiating

```
fn build_user(username::String, email::String) -> User{
    User {
        username,
        email,
        active: true,
    }
}
```

- you can also reference already created fields from other structs:

```
let user2 = User {
    ...user1,
    active: false,
};
or
{
    username: user1.username,
    email: user1.email,
    active:false,
};
```

- You can use tuples to create a struct:

```
Struct Color(i32,i32,i32);
Struct Point(i32,i32,i32);
...
let black = Color(255,255,255);
let origin = Point(0,0,0);
```

- Advanced topics:

  - You can create a struct without any field. They are called unit-like structs. They can be used as a trait of data of some type.

  ```
  struct AlwaysEqual;
  ...
  let subject = AlwaysEqual;

  ```

  - Ownership: You cannot create structs with references without error. Chapter 10 will talk about how to fix the errors.

  ```
  struct User {
    email: &str, // error: this will generate a expected name lifetime parameter error
    username: &str,// error: this will generate a expected name lifetime parameter error
  };

  ...
  let user = User{
  email: "someemail@email.com",
  username: "user1",
  };
  ```

##### 5. 2. Methods

- Methods are like functions but defined in the context of a struct.
- defining a method starts with the word `impl`.
- When we want to pass in the structure in question as the **first** parameter, we use the notation `&self` in the parameter.
  Whatever happen here we don't mutate the the struct itself.
- If we want to mutate we use a mutable reference `&mut self`
- we start the function inside the `impl <struct_name>{}` with self.
- we can define more than one function inside here
- we can have multiple `impl` block as well of the same struct.
- **Associated functions**: These can be methods or functions that are associated with the structure in question and are called using `<structname>::<method>`. More in chapter 7.
- Associated functions that aren't methods are often use as constructors that will return a new instance of the struct.
- They are called assosciated functions because they don't need self as the first parameter.
