extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::net::SocketAddr;
use std::thread;

mod opt;

fn main() {
    let mut server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34254)),
        buf: [0; 1000]
    };
    let socket = server.bind();

    loop {
        let client = server.receive(socket.try_clone().expect("failed to clone socket"));
        println!("{:?} from Client, usize: {:?}", client.ipaddr, client.size);
        let control_info = opt::ControlInfo::deserialize(client.buf);
        println!("ControlInfo: {:?}", control_info);

        thread::spawn(move || {
            let mut send_server = opt::Server {
                ipaddr: SocketAddr::from(([127, 0, 0, 1], 30000)),
                buf: [0; 1000]
            };
            let send_socket = send_server.bind();
            let byte = opt::ControlInfo::serialize(control_info);
            send_socket.send_to(&byte, SocketAddr::from(([127, 0, 0, 1], 20202))).expect("couldn't send data");
        });
    }
}

#[test]
fn mock_client_and_mock_data_server() {
    let handle = thread::spawn(|| {
        // 仮のデータサーバを立てる
        let mut server = opt::Server {
            ipaddr: SocketAddr::from(([127, 0, 0, 1], 20202)),
            buf: [0; 1000]
        };
        let socket = server.bind();

        // 仮のクライアントからデータを送信
        let client = std::net::UdpSocket::bind("127.0.0.1:12345").expect("couldn't bind to address");
        let control_info = opt::ControlInfo {
            character_id: "aworld client".to_string(),
            button_name: "enter".to_string(),
            status: true,
            optional: "hoo,bar".to_string()
        };
        let byte = opt::ControlInfo::serialize(control_info);
        client.send_to(&byte, "127.0.0.1:34254").expect("couldn't send data");
        println!("Send from Client");

        // コントロールサーバーからの受信
        loop {
            let client = server.receive(socket.try_clone().expect("failed to clone socket"));
            println!("{:?} from Control Server, usize: {:?}, buf: {:?}", client.ipaddr, client.size, client.buf);
        }
    });

    assert!(handle.join().is_ok());
}

