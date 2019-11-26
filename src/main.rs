extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::value::Value;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, RwLock};
use std::thread;

mod opt;

#[derive(Debug, Clone)]
pub struct Button {
    pub status: bool,
    pub optional: Value,
}

impl Button {
    fn new() -> Self {
        Self {
            status: false,
            optional: Value::Null,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub login: Button,
    pub up: Button,
    pub down: Button,
    pub right: Button,
    pub left: Button,
    pub spacebar: Button
}

impl Controller {
    fn new() -> Self {
        Self {
            login: Button::new(),
            up: Button::new(),
            down: Button::new(),
            right: Button::new(),
            left: Button::new(),
            spacebar: Button::new(),
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a Controller {
    type Item = (String, bool, Value);
    type IntoIter = ControllerIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ControllerIntoIterator {
            controller: self,
            index: 0,
        }
    }
}

pub struct ControllerIntoIterator<'a> {
    controller: &'a Controller,
    index: usize,
}

impl<'a> std::iter::Iterator for ControllerIntoIterator<'a> {
    type Item = (String, bool, Value);
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => (
                "login".to_owned(),
                self.controller.login.status,
                self.controller.login.optional.clone(),
            ),
            1 => (
                "forward".to_owned(),
                self.controller.up.status,
                self.controller.up.optional.clone(),
            ),
            2 => (
                "backward".to_owned(),
                self.controller.down.status,
                self.controller.down.optional.clone(),
            ),
            3 => (
                "turn_right".to_owned(),
                self.controller.right.status,
                self.controller.right.optional.clone(),
            ),
            4 => (
                "turn_left".to_owned(),
                self.controller.left.status,
                self.controller.left.optional.clone(),
            ),
            5 => (
                "pickup".to_owned(),
                self.controller.spacebar.status,
                self.controller.spacebar.optional.clone(),
            ),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

fn main() {
    let mut server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34255)),
        buf: [0; 4096],
    };
    let socket = server.bind();
    let controllers: Arc<RwLock<HashMap<(String, i64), Arc<RwLock<Controller>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let controllers2 = controllers.clone();

    let mut send_server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34250)),
        buf: [0; 4096],
    };
    let send_socket = send_server.bind();

    thread::spawn(move || loop {
        let controllers_lock = controllers2.read().unwrap();
        for ((ip, salt), controller) in controllers_lock.iter() {
            let mut is_login = false;
            {
                let lock = controller.read().unwrap();
                for (kind, should_send, optional) in lock.into_iter() {
                    if should_send {
                        if kind == "login" {
                            is_login = true;
                        }
                        let buf = format!(
                            r#"{{
                    "salt": {},
                    "addr": "{}",
                    "kind": "{}",
                    "payload": {}}}"#,
                            salt, ip, kind, optional
                        );
                        send_socket
                            .send_to(buf.as_bytes(), SocketAddr::from(([127, 0, 0, 1], 34254)))
                            .expect("couldn't send data");
                    }
                }
            }
            if is_login {
                controller.write().unwrap().login.status = false;
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
        let salt = control_info.salt;
        let ip_is_not_found = !controllers
            .read()
            .unwrap()
            .contains_key(&(ip.clone(), salt));
        if ip_is_not_found {
            controllers
                .write()
                .unwrap()
                .insert((ip.clone(), salt), Arc::new(RwLock::new(Controller::new())));
        }
        println!("{}", ip_is_not_found);
        if let Some(controller) = controllers.read().unwrap().get(&(ip, salt)) {
            match &*control_info.button_name {
                "login" => {
                    let mut lock = controller.write().unwrap();
                    lock.login.status = control_info.status;
                    lock.login.optional = control_info.optional;
                }
                "up" => {
                    let mut lock = controller.write().unwrap();
                    lock.up.status = control_info.status;
                    lock.up.optional = control_info.optional;
                }
                "down" => {
                    let mut lock = controller.write().unwrap();
                    lock.down.status = control_info.status;
                    lock.down.optional = control_info.optional;
                }
                "left" => {
                    let mut lock = controller.write().unwrap();
                    lock.left.status = control_info.status;
                    lock.left.optional = control_info.optional;
                }
                "right" => {
                    let mut lock = controller.write().unwrap();
                    lock.right.status = control_info.status;
                    lock.right.optional = control_info.optional;
                }
                "spacebar" => {
                    let mut lock = controller.write().unwrap();
                    lock.spacebar.status = control_info.status;
                    lock.spacebar.optional = control_info.optional;
                }
                _ => {}
            }
        }
        println!("update");
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
