use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Command;
use threadpool::ThreadPool;

use client_server::build_packet;

const ADDRESS: &str = "127.0.0.1";
const PORT: &str = "7878";

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

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection accepted!");

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut request_len: [u8; 2] = [0, 0];
    // let mut request_bytes: [u8; 255] = [" ".as_bytes()[0]; 255];

    loop {
        println!("Loop iteration");
        match stream.read_exact(&mut request_len) {
            Ok(_) => {
                let request_len = u16::from_ne_bytes(request_len) as usize;
                let mut request = vec![0; request_len];
                stream.read_exact(&mut request).unwrap();
                let mut request_str = std::str::from_utf8(&request).unwrap().trim();

                println!("Received message: {}", request_str);

                match request_str {
                    "get_once" => {
                        let packet = build_packet(fetch_info().as_bytes());
                        stream
                            .write_all(&packet)
                            .expect("Failed at writing onto the stream");
                        
                    }
                    "get_stream" => {
                        // let mut buffer = std::io::BufReader::new(stream);
                        let mut request_bytes: [u8; 255] = [" ".as_bytes()[0]; 255];

                        while request_str != "stop" {
                            // stream.read_to_string(&mut buffer).expect("Looping read failed");
                            stream.read_exact(&mut request_bytes).unwrap();
                            request_str = std::str::from_utf8(&request_bytes).unwrap().trim();
                            stream
                                .write_all(fetch_info().as_bytes())
                                .expect("Looping write failed");
                        }
                        
                    }
                    _ => (),
                }
            }
            Err(_) => {
                println!("Handling connection over");
                break
            },
        }
    }

    // }
}

fn fetch_info() -> String {
    // mouse pointer info
    let mut info = xdotool::mouse::get_mouse_location().stdout;

    // keyboard descriptor info
    info.append(
        &mut Command::new("setxkbmap")
            .arg("-query")
            .output()
            .expect("Command execution failed")
            .stdout,
    );

    info.push(String::from("\n").as_bytes()[0]);

    String::from_utf8(info).unwrap()
}