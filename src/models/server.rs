use serde_json::value::Value;

use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str;

pub struct Server {
    pub ipaddr: SocketAddr,
    pub buf: [u8; 4096],
    pub socket: UdpSocket
}

impl Server {
    pub fn new(ip: &str, buf: [u8; 4096]) -> Server {
        let socket_addr: SocketAddr = ip
            .parse()
            .expect("Unable to parse socket address");
        let ipaddr = SocketAddr::from(socket_addr);
        return Server {
            ipaddr,
            buf,
            socket: UdpSocket::bind(ipaddr).expect("couldn't bind to address"),
        }
    }

    pub fn receive(&mut self, socket: UdpSocket) -> Client {
        self.buf = [0; 4096];
        let (number_of_bytes, src_addr) = socket
            .recv_from(&mut self.buf)
            .expect("Didn't receive data");

        return Client {
            ipaddr: src_addr,
            size: number_of_bytes,
            buf: self.buf.to_vec(),
        };
    }

    pub fn send(&mut self, ip: &str, buf: String) {
        let socket_addr: SocketAddr = ip
            .parse()
            .expect("Unable to parse socket address");
        let ipaddr = SocketAddr::from(socket_addr);
        self.socket
            .send_to(buf.as_bytes(), ipaddr)
            .expect("couldn't send data");
    }
}

pub struct Client {
    pub ipaddr: SocketAddr,
    pub size: usize,
    pub buf: Vec<u8>,
}

impl Client {
    pub fn ip_str(&self) -> String {
        return match self.ipaddr {
            SocketAddr::V4(v4) => v4.to_string(),
            SocketAddr::V6(v6) => v6.to_string(),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlInfo {
    pub salt: i64,
    pub character_id: String,
    pub button_name: String,
    pub status: bool,
    pub optional: Value,
}

impl ControlInfo {
    pub fn deserialize(buf: Vec<u8>) -> ControlInfo {
        let control_info_str = str::from_utf8(&buf).expect("Found invalid UTF-8");
        println!("{}", control_info_str);
        // JSON has non-whitespace trailing characters after the value.
        // https://stackoverflow.com/questions/56817010/why-do-i-always-get-a-trailing-characters-error-when-trying-to-parse-data-with
        let control_info_trimed_str = control_info_str.trim_matches(char::from(0));
        let control_info: ControlInfo = serde_json::from_str(control_info_trimed_str).unwrap();
        control_info
    }

    pub fn serialize(control_info: ControlInfo) -> Vec<u8> {
        let json = serde_json::to_string(&control_info).unwrap();
        let byte = json.as_bytes();
        return byte.to_vec();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attack {
    pub character_id: String,
    pub action: String,
    pub payload: String,
}

impl Attack {
    pub fn serialize(attack: Attack) -> Vec<u8> {
        let json = serde_json::to_string(&attack).unwrap();
        let byte = json.as_bytes();
        return byte.to_vec();
    }
}
