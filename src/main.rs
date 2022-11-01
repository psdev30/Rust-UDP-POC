mod client;
mod host;

use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::client::{client, create_client};
use crate::host::{create_host, host};

#[allow(dead_code)]
#[allow(unused)]

const TOGGLE: &str = "h";
const SIZE: usize = 50;


fn main() {
    let (sx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let (stx, rrx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let udp_conn = create_host();
    let udp_conn_client = create_client();

    thread::spawn(move || {
        client(udp_conn_client, sx, rrx);
    });

    let ip_addr_client = get_ip_addr_client();
    udp_conn.connect(ip_addr_client);

    udp_conn.send(b"testmsg").expect("couldn't send message");

    loop {
        let msg = rx.recv().unwrap();
        println!("{}", msg);
    }
}

fn get_ip_addr() -> SocketAddr {
    let my_local_ip = local_ip().unwrap();
    let mut ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    if let IpAddr::V4(ipv4) = my_local_ip {
        ip_addr = ipv4;
    }
    let socket = SocketAddr::new(IpAddr::from(ip_addr), 8080);
    socket
}

fn get_ip_addr_client() -> SocketAddr {
    let my_local_ip = local_ip().unwrap();
    let mut ip_addr = Ipv4Addr::new(127, 0, 0, 1);
    if let IpAddr::V4(ipv4) = my_local_ip {
        ip_addr = ipv4;
    }
    let socket = SocketAddr::new(IpAddr::from(ip_addr), 9800);
    socket
}
