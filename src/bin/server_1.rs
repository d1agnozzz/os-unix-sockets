use byteorder::{NativeEndian, ReadBytesExt};
use std::io::{BufRead, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::process::Command;
use chrono::Local;
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
        // stream.set_nonblocking(true).unwrap();

        println!("Connection accepted!");

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let mut request_bytes: [u8; 255] = [" ".as_bytes()[0]; 255];


    loop {
        println!("Loop iteration");
        match stream.read_u16::<NativeEndian>() {
            Ok(request_len) => {
                let mut request_body = vec![0; request_len as usize];
                stream.read_exact(&mut request_body).unwrap();
                let mut request_str = std::str::from_utf8(&request_body).unwrap().trim();

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
                        // let mut request_bytes: [u8; 255] = [" ".as_bytes()[0]; 255];
                        let mut prev_info = fetch_info();
                        let packet = build_packet(prev_info.as_bytes());
                        // stream.read_to_string(&mut buffer).expect("Looping read failed");
                        stream.write_all(&packet).expect("Looping write failed");

                        loop {
                            let info = fetch_info();
                            if info != prev_info {
                                println!("Sending reaction");
                                let packet = build_packet(info.as_bytes());
                                // stream.read_to_string(&mut buffer).expect("Looping read failed");
                                if stream.write_all(&packet).is_err() {
                                    break;
                                };
                            }
                            prev_info = info;
                        }
                    }
                    _ => (),
                }
            }
            Err(_) => {
                println!("Handling connection over");
                break;
            }
        }
    }

    // }
}

fn fetch_info() -> String {


    let mut info = format!("{}\n", Local::now().format("%H:%M:%S")).as_bytes().to_owned();

    // mouse pointer info
    info.append(&mut xdotool::mouse::get_mouse_location().stdout);

    // keyboard descriptor info
    info.append(
        &mut Command::new("setxkbmap")
            .arg("-query")
            .output()
            .expect("Command execution failed")
            .stdout,
    );

    String::from_utf8(info).unwrap()
}
