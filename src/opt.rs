use std::net::UdpSocket;
use std::net::SocketAddr;
use std::str;

pub struct Server {
    pub ipaddr: SocketAddr,
    pub buf: [u8; 1000]
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
            buf: self.buf.to_vec()
        }
    }
}

pub struct Client {
    pub ipaddr: SocketAddr,
    pub size: usize,
    pub buf: Vec<u8>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ControlInfo {
    pub character_id: String,
    pub button_name: String,
    pub status: bool,
    pub optional: String
}

impl ControlInfo {
    pub fn deserialize(buf: Vec<u8>) -> ControlInfo {
        let control_info_str = str::from_utf8(&buf).expect("Found invalid UTF-8");
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
    pub pyload: String
}

impl Attack {
    pub fn serialize(attack: Attack) -> Vec<u8> {
        let json = serde_json::to_string(&attack).unwrap();
        let byte = json.as_bytes();
        return byte.to_vec();
    }
}
