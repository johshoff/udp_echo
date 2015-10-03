extern crate mio;

use std::thread;
use mio::udp::*;
use mio::buf::SliceBuf;

fn main() {
    let address = "0.0.0.0:9999".parse().unwrap();
    let socket = UdpSocket::v4().unwrap();
    socket.bind(&address).unwrap();

    let mut buffer = Vec::new();
    loop {
        let result = socket.recv_from(&mut buffer);
        if let Ok(Some(from_address)) = result {
            println!("Got {} bytes from {:?}: {:?}", buffer.len(), from_address, buffer);

            let result = socket.send_to(&mut SliceBuf::wrap(&buffer), &from_address);
            println!("Result from echoing back: {:?}", result);

            buffer.clear();
        }

        thread::sleep_ms(10);
    }
}

