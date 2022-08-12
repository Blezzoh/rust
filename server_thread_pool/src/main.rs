use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use threadpool::ThreadPool;
use std::time::Duration;
fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(8);

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        pool.execute(||{
            handle_connection(stream);
        });
        println!("connection established");
    }
    println!("Hello, world!");
}

 
fn handle_connection(mut stream:TcpStream){

    let mut buffer = [0;1024];

    stream.read(&mut buffer).unwrap();

    let home = b"GET / HTTP/1.1\r\n";

    let path_one = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(home){
        ("HTTP/1.1 200 OK","index.html")
    }
    else if buffer.starts_with(path_one){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK","path_one.html")
    }
    else{
        ("HTTP/1.1 400 NOT FOUND", "404.html")

    };

    let contents = fs::read_to_string(filename).unwrap();

    let response =format!( 
        "{}\r\n Content-Length{}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
