use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

use aworld_server::models::controller::Controller;
use aworld_server::models::server::{ControlInfo, Server};

fn main() {
    let mut receiving_server = Server::new("127.0.0.1:34255", [0; 4096]);
    let mut sending_server = Server::new("127.0.0.1:34250", [0; 4096]);

    let controllers: Arc<RwLock<HashMap<(String, i64), Arc<RwLock<Controller>>>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let controllers2 = controllers.clone();

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
                        sending_server.send("127.0.0.1:34254", buf)
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
        let client = receiving_server.receive(
            receiving_server
                .socket
                .try_clone()
                .expect("failed to clone socket"),
        );
        println!("{:?} from Client, usize: {:?}", client.ipaddr, client.size);
        let ip = client.ip_str();
        let control_info = ControlInfo::deserialize(client.buf);
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
                "a" => {
                    let mut lock = controller.write().unwrap();
                    lock.a.status = control_info.status;
                    lock.a.optional = control_info.optional;
                }
                "i" => {
                    let mut lock = controller.write().unwrap();
                    lock.i.status = control_info.status;
                    lock.i.optional = control_info.optional;
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
        let mut server = Server::new("127.0.0.1:34255", [0; 4096]);

        // 仮のクライアントからデータを送信
        let client =
            std::net::UdpSocket::bind("127.0.0.1:12345").expect("couldn't bind to address");
        let control_info = ControlInfo {
            salt: 123456789,
            character_id: "aworld client".to_string(),
            button_name: "enter".to_string(),
            status: true,
            optional: serde_json::Value::String("hoo,bar".to_string()),
        };
        let byte = ControlInfo::serialize(control_info);
        client
            .send_to(&byte, "127.0.0.1:34254")
            .expect("couldn't send data");
        println!("Send from Client");

        // コントロールサーバーからの受信
        loop {
            let client = server.receive(server.socket.try_clone().expect("failed to clone socket"));
            println!(
                "{:?} from Control Server, usize: {:?}, buf: {:?}",
                client.ipaddr, client.size, client.buf
            );
        }
    });

    assert!(handle.join().is_ok());
}
