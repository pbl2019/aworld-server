use std::net::UdpSocket;
use std::net::SocketAddr;

struct Server {
    ipaddr: SocketAddr,
    buf: [u8; 10]
}

impl Server {
    fn bind(&mut self) -> UdpSocket {
        return UdpSocket::bind(self.ipaddr).expect("couldn't bind to address");
    }

    fn reseive(&mut self, socket: UdpSocket) -> Client {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut self.buf).expect("Didn't receive data");

        return Client {
            ipaddr: src_addr,
            size: number_of_bytes,
            buf: self.buf
        }
    }
}

struct Client {
    ipaddr: SocketAddr,
    size: usize,
    buf: [u8; 10]
}

fn main() {
    let mut server = Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34254)),
        buf: [0; 10]
    };
    let socket = server.bind();
    loop {
        let client = server.reseive(socket.try_clone().expect("failed to clone socket"));
        println!("{:?} from message, usize: {:?}, buf: {:?}", client.ipaddr, client.size, client.buf);
    }
}

#[test]
fn it_works() {
    let socket = UdpSocket::bind("127.0.0.1:12345").expect("couldn't bind to address");
    socket.send_to(&[1, 2, 5], "127.0.0.1:34254").expect("couldn't send data");
}
