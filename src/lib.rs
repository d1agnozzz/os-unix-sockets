
pub fn build_packet(content: &[u8]) -> Vec<u8> {
    let packet_len = content.len();

    let mut packet = vec![0; 2 + packet_len];
    packet[0] = packet_len.to_ne_bytes()[0];
    packet[1] = packet_len.to_ne_bytes()[1];

    for (i, v) in content.iter().enumerate() {
        packet[i + 2] = *v;
    }
    packet
}

use std::{net::TcpStream, io::Read};

use byteorder::{ReadBytesExt, BigEndian, NativeEndian};
pub fn  read_and_decode_packet(mut stream: TcpStream) -> std::io::Result<String> {
    let packet_len = stream.read_u16::<NativeEndian>()?;
    let mut buffer = vec![0; packet_len as usize];
    stream.read_exact(&mut buffer)?;

    match String::from_utf8(buffer){
        Ok(s) => Ok(s),
        Err(_) => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
    }

}
