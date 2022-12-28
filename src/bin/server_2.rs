use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::process::CommandExt;
use std::process::Command;

use threadpool::ThreadPool;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "8787";

fn main() {
    let socket_addr = format!("{}:{}", ADDRESS, PORT);

    let listener = match TcpListener::bind(&socket_addr) {
        Ok(tcp_listener) => tcp_listener,
        Err(_) => {
            println!(
                "Could not bind to an {}. Maybe server is already running?\nExiting...",
                socket_addr
            );
            std::process::exit(0);
        }
    };

    let pool = ThreadPool::new(8);

    println!(
        "Server is now waiting for connections at {}!\n",
        listener.local_addr().unwrap()
    );

    // for i in 0..3 {
    //     std::thread::spawn(|| {
    //         std::thread::sleep(std::time::Duration::from_secs(3));
    //     });
    // }

    let out = Command::new("ps")
        .arg("-T")
        .arg("-l")
        .arg("-p")
        .arg(format!("{}", std::process::id()))
        .output()
        .expect("Command execution failed");
    println!("{}", String::from_utf8(out.stdout).unwrap());
    println!("{}", std::process::id());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = std::io::BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next();
    let mut message = String::new();
    stream
        .read_to_string(&mut message)
        .expect("Failed at reading the unix stream");

    println!("Received message: {message}\nReplying... ");

    stream
        .write_all(b"I hear you")
        .expect("Failed at writing onto the stream");
}

fn fetch_info() -> String {
    let pid = std::process::id();

    todo!()
}
