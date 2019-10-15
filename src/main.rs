use std::net::SocketAddr;
use std::thread;

mod opt;

fn main() {
    let mut server = opt::Server {
        ipaddr: SocketAddr::from(([127, 0, 0, 1], 34254)),
        buf: [0; 10]
    };
    let socket = server.bind();

    loop {
        let client = server.receive(socket.try_clone().expect("failed to clone socket"));
        println!("{:?} from Client, usize: {:?}, buf: {:?}", client.ipaddr, client.size, client.buf);

        thread::spawn(move || {
            let mut send_server = opt::Server {
                ipaddr: SocketAddr::from(([127, 0, 0, 1], 30000)),
                buf: [0; 10]
            };
            let send_socket = send_server.bind();
            send_socket.send_to(&[0; 10], SocketAddr::from(([127, 0, 0, 1], 20202))).expect("couldn't send data");
        });
    }
}

#[test]
fn mock_data_server() {
    let handle = thread::spawn(|| {
        // 仮のデータサーバを立てる
        let mut server = opt::Server {
            ipaddr: SocketAddr::from(([127, 0, 0, 1], 20202)),
            buf: [0; 10]
        };
        let socket = server.bind();

        // 仮のクライアントからデータを送信
        let client = std::net::UdpSocket::bind("127.0.0.1:12345").expect("couldn't bind to address");
        client.send_to(&[1, 2, 5], "127.0.0.1:34254").expect("couldn't send data");
        println!("Send from Client");

        // コントロールサーバーからの受信
        loop {
            let client = server.receive(socket.try_clone().expect("failed to clone socket"));
            println!("{:?} from Control Server, usize: {:?}, buf: {:?}", client.ipaddr, client.size, client.buf);
        }
    });

    assert!(handle.join().is_ok());
}

