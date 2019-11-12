extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::thread;

mod opt;

#[derive(Debug, Copy, Clone)]
pub struct Controller {
    pub login: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
}

impl Controller {
    fn new() -> Self {
        Self {
            login: false,
            up: false,
            down: false,
            right: false,
            left: false,
        }
    }
}

impl std::iter::IntoIterator for Controller {
    type Item = (String, bool);
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            ("login".to_owned(), self.login),
            ("forward".to_owned(), self.up),
            ("backward".to_owned(), self.down),
            ("turn_right".to_owned(), self.right),
            ("turn_left".to_owned(), self.left),
        ]
        .into_iter()
    }
}

fn main() {
    let mut server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34255)),
        buf: [0; 4096],
    };
    let socket = server.bind();
    let controllers: Arc<RwLock<HashMap<String, Arc<RwLock<Controller>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let controllers2 = controllers.clone();

    let mut send_server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34250)),
        buf: [0; 4096],
    };
    let send_socket = send_server.bind();

    thread::spawn(move || loop {
        let controllers_lock = controllers2.read().unwrap();
        for (ip, controller) in controllers_lock.iter() {
            let mut lock = controller.write().unwrap();
            for (kind, should_send) in lock.into_iter() {
                if should_send {
                    if kind == "login" {
                        lock.login = false;
                    }
                    let buf = format!(
                        r#"{{
                    "addr": "{}",
                    "kind": "{}",
                    "payload": {{ "speed": 1.0, "angle": 0.1 }}
                }}"#,
                        ip, kind
                    );
                    send_socket
                        .send_to(buf.as_bytes(), SocketAddr::from(([127, 0, 0, 1], 34254)))
                        .expect("couldn't send data");
                }
            }
        }
        thread::sleep_ms(5);
    });

    loop {
        let client = server.receive(socket.try_clone().expect("failed to clone socket"));
        println!("{:?} from Client, usize: {:?}", client.ipaddr, client.size);
        let ip = match client.ipaddr {
            SocketAddr::V4(v4) => v4.to_string(),
            SocketAddr::V6(v6) => v6.to_string(),
        };
        let control_info = opt::ControlInfo::deserialize(client.buf);
        println!("ControlInfo: {:?}", control_info);
        let ip_is_not_found = !controllers.read().unwrap().contains_key(&ip);
        if ip_is_not_found {
            controllers
                .write()
                .unwrap()
                .insert(ip.clone(), Arc::new(RwLock::new(Controller::new())));
        }
        if let Some(controller) = controllers.read().unwrap().get(&ip) {
            match &*control_info.button_name {
                "login" => {
                    controller.write().unwrap().login = control_info.status;
                }
                "up" => {
                    controller.write().unwrap().up = control_info.status;
                }
                "down" => {
                    controller.write().unwrap().down = control_info.status;
                }
                "left" => {
                    controller.write().unwrap().left = control_info.status;
                }
                "right" => {
                    controller.write().unwrap().right = control_info.status;
                }
                _ => {}
            }
        }
    }
}

#[test]
fn mock_client_and_mock_data_server() {
    let handle = thread::spawn(|| {
        // 仮のデータサーバを立てる
        let mut server = opt::Server {
            ipaddr: SocketAddr::from(([127, 0, 0, 1], 20202)),
            buf: [0; 4096],
        };
        let socket = server.bind();

        // 仮のクライアントからデータを送信
        let client =
            std::net::UdpSocket::bind("127.0.0.1:12345").expect("couldn't bind to address");
        let control_info = opt::ControlInfo {
            character_id: "aworld client".to_string(),
            button_name: "enter".to_string(),
            status: true,
            optional: "hoo,bar".to_string(),
        };
        let byte = opt::ControlInfo::serialize(control_info);
        client
            .send_to(&byte, "127.0.0.1:34254")
            .expect("couldn't send data");
        println!("Send from Client");

        // コントロールサーバーからの受信
        loop {
            let client = server.receive(socket.try_clone().expect("failed to clone socket"));
            println!(
                "{:?} from Control Server, usize: {:?}, buf: {:?}",
                client.ipaddr, client.size, client.buf
            );
        }
    });

    assert!(handle.join().is_ok());
}
