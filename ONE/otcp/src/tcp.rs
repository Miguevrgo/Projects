use crate::state::TcpState;
use rand::random;
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4},
};

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

impl TcpHeader {
    fn from(
        s_port: u16,
        d_port: u16,
        seq: u32,
        ack: u32,
        o_r_c: u16,
        window: u16,
        u_pointer: u16,
    ) -> TcpHeader {
        TcpHeader {
            source_port: s_port,
            destination_port: d_port,
            seq_number: seq,
            ack_number: ack,
            offset_reserv_control: o_r_c,
            window,
            checksum: Self::checksum(),
            urgent_pointer: u_pointer,
        }
    }

    fn checksum() -> u16 {
        unimplemented!()
    }

    fn compose_offset_reserv_control(offset: u8, flags: u8) -> u16 {
        ((offset as u16) << 12) | (flags as u16)
    }
}

pub struct TcpStream {
    state: TcpState,
    local_address: Ipv4Addr,
    local_port: u16,
    remote_address: Option<Ipv4Addr>,
    remote_port: Option<u16>,
    ack_number: u32,
    seq_number: u32,
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
            local_address: addr_ip,
            local_port: addr_port,
            remote_address: None,
            remote_port: None,
            seq_number: 0,
            ack_number: 0,
        })
    }

    pub fn connect(&mut self, r_port: u16, r_address: Ipv4Addr) -> io::Result<()> {
        if self.state != TcpState::Closed {
            panic!("Implement logic (RFC 793 says something about this in strange ways :O )")
        }

        let socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::TCP))?;
        socket.set_reuse_address(true)?;
        let local_socket = SocketAddrV4::new(self.local_address, self.local_port);
        socket.bind(&local_socket.into())?;

        self.seq_number = rand::random::<u32>();
        self.remote_port = Some(r_port);
        self.remote_address = Some(r_address);
        let flags = 0b00010; // SYN Flag
        let header = TcpHeader::from(
            self.local_port,
            self.remote_port.unwrap(),
            self.seq_number,
            self.ack_number,
            TcpHeader::compose_offset_reserv_control(0, flags),
            8192,
            0,
        );

        let tcp_segment = self.serialize_header(&header)?;
        socket.send_to(&tcp_segment, &SocketAddrV4::new(r_address, r_port).into())?;
        self.state = TcpState::SynSent;
        Ok(())
    }

    fn serialize_header(&self, header: &TcpHeader) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(20);
        bytes.extend_from_slice(&header.source_port.to_be_bytes());
        bytes.extend_from_slice(&header.destination_port.to_be_bytes());
        bytes.extend_from_slice(&header.seq_number.to_be_bytes());
        bytes.extend_from_slice(&header.ack_number.to_be_bytes());
        bytes.extend_from_slice(&header.offset_reserv_control.to_be_bytes());
        bytes.extend_from_slice(&header.window.to_be_bytes());
        bytes.extend_from_slice(&header.checksum.to_be_bytes());
        bytes.extend_from_slice(&header.urgent_pointer.to_be_bytes());
        Ok(bytes)
    }
}

#[test]
fn connect() {
    let stream = TcpStream::new("192.168.1.1:80");
    assert!(stream.is_ok());
    let stream = stream.unwrap();
    assert_eq!(Ok(stream.local_address), "192.168.1.1".parse());
    assert_eq!(stream.local_port, 80);
}
