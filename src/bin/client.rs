use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    let mut unix_stream =
        UnixStream::connect(socket_path).context("Could not connect to socket")?;

    write_request_and_shutdown(&mut unix_stream)?;
    read_from_stream(&mut unix_stream)?;
    loop {
        
    }
    Ok(())
}

fn write_request_and_shutdown(unix_stream : &mut UnixStream) -> anyhow::Result<()> {
    unix_stream
        .write(b"Hello server!")
        .context("Failed aat writing onto the stream")?;

    println!("Request sent");
    println!("Shutting down writing on the stream, waiting for response...");

    unix_stream
        .shutdown(std::net::Shutdown::Write)
        .context("Could not shutdown writing on the stream")?;

    Ok(())
}

fn read_from_stream(unix_stream: &mut UnixStream) -> anyhow::Result<()> {
    let mut response = String::new();
    unix_stream
        .read_to_string(&mut response)
        .context("Failed at reading the unix stream")?;

    println!("We received this response: {}", response);
    Ok(())
}
