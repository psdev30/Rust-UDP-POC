use std::net::UdpSocket;
use std::sync::mpsc::{Receiver, Sender};
use crate::{get_ip_addr, get_ip_addr_player2};

const SIZE: usize = 50;

pub(crate) fn create_player1_connection() -> UdpSocket {
    let ip_addr = get_ip_addr();
    let udp_conn = UdpSocket::bind(ip_addr).unwrap();
    udp_conn
}

pub(crate) fn player1(socket: UdpSocket, tx: Sender<String>, erx: Receiver<String>) -> std::io::Result<()>
{
    println!("Player 1 connection active:");

    loop {
        let mut buf = [0 as u8; SIZE];
        let (num_bytes, src_addr) = socket.recv_from(&mut buf)?;
        let msg = String::from_utf8((&buf[0..num_bytes]).to_vec()).unwrap();

        //send client data through channel
        match tx.send(String::from(msg)){
            Ok(_) => {
                //no issues sending msg
            }
            Err(e) => {
                println!("Error sending message: {}", e)
            }
        }


        //see if there are any messages to send a client
        match erx.try_recv() {
            Ok(msg) => {
                let e = String::from("Error: ".to_owned() + &msg);
                socket.send_to(e.as_bytes(), src_addr);
            }
            Err(e) => {
                //no message in queue, do nothing
            }
        }

    }
}