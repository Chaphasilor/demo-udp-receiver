use std::time::{SystemTime};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    loop {
        let socket = UdpSocket::bind("127.0.0.1:34001")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 8];
        println!("waiting for packet to arrive...");
        let (_message_length, src) = socket.recv_from(&mut buf)?;

        let sender_time = std::time::Duration::from_micros(u64::from_be_bytes(buf));

        let start = SystemTime::now();
        let time = start.duration_since(std::time::UNIX_EPOCH).expect("Couldn't get system time");
        buf = (time.as_micros() as u64).to_be_bytes();

        // send current system time back to sender
        socket.send_to(&buf, &src)?;

        let received_after = (time - sender_time).as_micros() as u64;

        println!("Received message after {} µs", received_after)
        
    } // the socket is closed here
}
