use std::{fs::File, io::{self, Read}};

// Result<T,io:Error> is our way of error handling
fn read_file_text(filename:&str)->Result<String, io::Error>{
    let f = File::open(filename);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// same function with shortcut "?""
fn read_file_text_two(filename:&str) ->Result<String, io::Error>{
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    let filename = "hello.txt";
    let does_not_exit = "text.txt";
    let result_one = read_file_text(filename).expect("problem reading file");
    let result_two = read_file_text_two(filename).expect("problem reading file");
    println!("file hello.txt using long read function:\n{}", result_one);
    println!("file hello.txt using shortcut '?' function:\n{}", result_two);

    read_file_text_two(does_not_exit).expect("problem reading file");
}
