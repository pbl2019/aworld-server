use std::net::UdpSocket;
use std::net::SocketAddr;

pub struct Server {
    pub ipaddr: SocketAddr,
    pub buf: [u8; 10]
}

impl Server {
    pub fn bind(&mut self) -> UdpSocket {
        return UdpSocket::bind(self.ipaddr).expect("couldn't bind to address");
    }

    pub fn receive(&mut self, socket: UdpSocket) -> Client {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut self.buf).expect("Didn't receive data");

        return Client {
            ipaddr: src_addr,
            size: number_of_bytes,
            buf: self.buf
        }
    }
}

pub struct Client {
    pub ipaddr: SocketAddr,
    pub size: usize,
    pub buf: [u8; 10]
}
