use std::io::{BufRead, Read, Write};
use std::net::TcpStream;
use std::os::unix::net::{UnixListener, UnixStream};

use byteorder::{NativeEndian, ReadBytesExt};
use egui::accesskit::ActionRequest;

use client_server::build_packet;

const ADDRESS: &str = "localhost";
const PORT: &str = "7878";

fn main() {
    let socket_addr = format!("{}:{}", ADDRESS, PORT);

    let mut stream = TcpStream::connect(socket_addr).expect("Could not connect to socket");

    // write_request(b"get_once\n", &mut stream);
    // read_from_stream(&mut stream);
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // write_request(b"get_once\n", &mut stream);
    // read_from_stream(&mut stream);
    // write_request(b"get_once\n", &mut stream);
    // read_from_stream(&mut stream);
    // write_request(b"get_once\n", &mut stream);
    // read_from_stream(&mut stream);
    // write_request(b"get_once\n", &mut stream);
    // read_from_stream(&mut stream);

    write_request(b"get_stream\n", &mut stream);
    loop {
        println!("{}", read_response(&mut stream));
    }

    // for i in 0..10 {
    //     let response_len = stream.read_u16::<NativeEndian>().expect("Looping size read failed") as usize;
    //     let mut response_body = vec![0; response_len];
    //     stream.read_exact(&mut response_body).expect("Looping body read failed");
    //     let response_str = std::str::from_utf8(&response_body).expect("Looping decode failed");
    //     println!("{i}\n{}", response_str);
    //     write_request(b"", &mut stream);
    // }
    // write_request(b"stop\n", &mut stream)
}

pub fn write_request(content: &[u8], stream: &mut TcpStream) {
    let request = build_packet(content);

    stream
        .write_all(&request)
        .expect("Failed at writing onto the stream");

    println!("Request sent");
    println!("Waiting for response...");
}

// fn build_packet(content: &[u8]) -> Vec<u8> {
//     let request_len = content.len();

//     let mut request = Vec::with_capacity(2 + request_len);
//     request[0] = request_len.to_ne_bytes()[0];
//     request[1] = request_len.to_ne_bytes()[1];

//     for (i, v) in content.iter().enumerate() {
//         request[i+2] = *v;
//     }
//     request
// }

pub fn read_response(stream: &mut TcpStream) -> String {
    let mut response_len: [u8; 2] = [0, 0];

    stream.read_exact(&mut response_len).unwrap();
    println!("Received response!");

    let response_len = u16::from_ne_bytes(response_len) as usize;

    let mut response_body = vec![0; response_len];

    stream.read_exact(&mut response_body).unwrap();

    String::from_utf8(response_body).unwrap()

    // println!("Buffer will be created");
    // let buf_reader = std::io::BufReader::new(&mut stream);

    // for line in buf_reader.lines() {
    //     println!("Received response: \n{}", line.unwrap())
    // }

    // let mut response = String::new();
    // stream
    //     .read_to_string(&mut response)
    //     .expect("Failed at reading the unix stream");

    // println!("We received this response: {}", response);
}
