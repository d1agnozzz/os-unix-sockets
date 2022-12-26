use std::os::unix::net::{UnixListener, UnixStream};
use std::io::{Read, Write};

use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let socket_path = "mysocket";

    delete_existing_socket(socket_path)?;

    let unix_listener = UnixListener::bind(socket_path).context("context")?;

    println!("Server is now waiting for connections!\n");

    let cap = 0;

    loop {
        let (mut unix_stream, socket_address) = unix_listener
            .accept()
            .context("Failed at accepting connection")?;
        handle_stream(unix_stream)?;
    }

    Ok(())
}

fn handle_stream(mut stream: UnixStream) -> anyhow::Result<()> {
    let mut message = String::new();
    stream.read_to_string(&mut message).context("Failed at reading the unix stream")?;

    println!("Received message: {message}\nReplying... ");

    stream.write(b"I hear you").context("Failed at writing onto the stream")?;

    Ok(())
}

fn delete_existing_socket(socket_path: &str) -> anyhow::Result<()> {
    if std::fs::metadata(socket_path).is_ok() {
        println!("Socket already exist. Deleting...");
        std::fs::remove_file(socket_path).with_context(|| {
            format!("Could not delete previous socket at {:?}", socket_path)
        })?;
    }
    Ok(())
}