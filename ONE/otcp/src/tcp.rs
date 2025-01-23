use crate::state::TcpState;
use socket2::{Domain, Protocol, Socket, Type};
use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4},
};

bitflags::bitflags! {
    pub struct TcpFlags: u8 {
        const FIN = 0x01;
        const SYN = 0x02;
        const RST = 0x04;
        const PSH = 0x08;
        const ACK = 0x10;
        const URG = 0x20;
    }
}

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
    reserved: u8,
    offset: u8,
    control: TcpFlags,
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
        offset: u8,
        control: TcpFlags,
        window: u16,
        u_pointer: u16,
    ) -> TcpHeader {
        TcpHeader {
            source_port: s_port,
            destination_port: d_port,
            seq_number: seq,
            ack_number: ack,
            offset,
            control,
            reserved: 0,
            window,
            checksum: 0,
            urgent_pointer: u_pointer,
        }
    }

    fn checksum(&self, src_ip: Ipv4Addr, dest_ip: Ipv4Addr) -> u16 {
        let mut buf = Vec::with_capacity(32);

        // Pseudo-header (RFC 793)
        buf.extend_from_slice(&src_ip.octets());
        buf.extend_from_slice(&dest_ip.octets());
        buf.push(0); // Reserved
        buf.push(6); // TCP
        buf.extend_from_slice(&(20u16).to_be_bytes()); // Longitud TCP

        buf.extend_from_slice(&self.source_port.to_be_bytes());
        buf.extend_from_slice(&self.destination_port.to_be_bytes());
        buf.extend_from_slice(&self.seq_number.to_be_bytes());
        buf.extend_from_slice(&self.ack_number.to_be_bytes());
        buf.push((self.offset << 4) | (self.reserved << 2));
        buf.push(self.control.bits());
        buf.extend_from_slice(&self.window.to_be_bytes());
        buf.extend_from_slice(&[0, 0]); // Checksum
        buf.extend_from_slice(&self.urgent_pointer.to_be_bytes());

        // Calculate checksum
        let mut sum = 0u32;
        for chunk in buf.chunks(2) {
            let word = u16::from_be_bytes([chunk[0], chunk.get(1).copied().unwrap_or(0)]);
            sum += u32::from(word);
        }

        while sum >> 16 != 0 {
            sum = (sum & 0xFFFF) + (sum >> 16);
        }

        !sum as u16
    }

    fn parse(data: &[u8]) -> io::Result<Self> {
        if data.len() < 20 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "TCP Header too short",
            ));
        }

        Ok(TcpHeader {
            source_port: u16::from_be_bytes([data[0], data[1]]),
            destination_port: u16::from_be_bytes([data[2], data[3]]),
            seq_number: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
            ack_number: u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
            offset: data[12] >> 4,
            reserved: (data[12] & 0x0F) >> 2, // 4 bits (offset) + 6 bits reservados
            control: TcpFlags::from_bits(data[13]).unwrap(),
            window: u16::from_be_bytes([data[14], data[15]]),
            checksum: u16::from_be_bytes([data[16], data[17]]),
            urgent_pointer: u16::from_be_bytes([data[18], data[19]]),
        })
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
        let flags = TcpFlags::SYN; // Usando TcpFlags::SYN
        let header = TcpHeader::from(
            self.local_port,
            self.remote_port.unwrap(),
            self.seq_number,
            self.ack_number,
            5, // Offset
            flags,
            8192,
            0,
        );

        let tcp_segment = self.serialize_header(&header)?;
        socket.send_to(&tcp_segment, &SocketAddrV4::new(r_address, r_port).into())?;
        self.state = TcpState::SynSent;
        // SYN sent, Next step in standard three way handshake is to receive an SYN + ACK
        let mut buf = [std::mem::MaybeUninit::uninit(); 1024];
        let (size, _) = socket.recv_from(&mut buf)?;
        let response = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, size) };

        let syn_ack_header = TcpHeader::parse(response)?;

        if !syn_ack_header
            .control
            .contains(TcpFlags::SYN | TcpFlags::ACK)
        {
            return Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "Respuesta inválida",
            ));
        }

        self.ack_number = syn_ack_header.seq_number.wrapping_add(1);
        self.seq_number = self.seq_number.wrapping_add(1);

        let ack_header = TcpHeader::from(
            self.local_port,
            r_port,
            self.seq_number,
            self.ack_number,
            5,
            TcpFlags::ACK,
            8192,
            0,
        );

        let ack_segment = self.serialize_header(&ack_header)?;
        socket.send_to(&ack_segment, &SocketAddrV4::new(r_address, r_port).into())?;

        self.state = TcpState::Established;
        Ok(())
    }

    fn serialize_header(&self, header: &TcpHeader) -> io::Result<Vec<u8>> {
        let mut bytes = Vec::with_capacity(20);
        bytes.extend_from_slice(&header.source_port.to_be_bytes());
        bytes.extend_from_slice(&header.destination_port.to_be_bytes());
        bytes.extend_from_slice(&header.seq_number.to_be_bytes());
        bytes.extend_from_slice(&header.ack_number.to_be_bytes());
        bytes.extend_from_slice(&header.offset.to_be_bytes());
        bytes.extend_from_slice(&[header.control.bits()]);
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
