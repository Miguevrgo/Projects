use std::{io, net::Ipv4Addr};

use crate::state::TcpState;

/// A TCP header follows the internet header, supplying information
/// specific to the TCP protocol, structure has been assigned according
/// to TCP 793, with the exact bits for each field, to achieve this
/// without the use of bitfields:
/// offset (4 bits) + reserved (6 bits) + Control bits (6 bits)
/// are arranged into a single field
pub struct TcpHeader {
    source_port: u16,
    destination_port: u16,
    seq_number: u32,
    ack_number: u32,
    offset_reserv_control: u16,
    window: u16,
    checksum: u16,
    urgent_pointer: u16,
}

pub struct TcpStream {
    state: TcpState,
    address: Ipv4Addr,
    port: u16,
}

impl TcpStream {
    pub fn new(addr: &str) -> io::Result<TcpStream> {
        let (ip, port) = addr.split_once(':').ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid address provided")
        })?;

        let addr_ip: Ipv4Addr = ip
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid Ip address"))?;
        let addr_port = port
            .parse::<u16>()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port"))?;

        Ok(TcpStream {
            state: TcpState::Closed,
            address: addr_ip,
            port: addr_port,
        })
    }

    pub fn connect(&mut self) {
        if self.state != TcpState::Closed {
            panic!("Implement logic (RFC 793 says something about this in strange ways :O )")
        }
        //TODO: Three way handshake
    }
}

#[test]
fn connect() {
    let stream = TcpStream::new("192.168.1.1:80");
    assert!(stream.is_ok());
    let stream = stream.unwrap();
    assert_eq!(Ok(stream.address), "192.168.1.1".parse());
    assert_eq!(stream.port, 80);
}
