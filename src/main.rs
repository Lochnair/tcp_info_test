mod tcp_info;

use std::error::Error;
use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::os::unix::io::AsRawFd;
use crate::tcp_info::get_tcp_info;

fn main() -> Result<(), Box<dyn Error>> {
    let ip_addr = IpAddr::from([195,88,54,16]);
    let addr = SocketAddr::new(ip_addr, 80);
    let mut tcp = std::net::TcpStream::connect(addr)?;

    let req = "GET / HTTP/1.0\nHost: vg.no\n\n";

    tcp.write(req.as_bytes())?;

    let mut buf: Vec<u8> =  vec![0; 100000];
    tcp.read(buf.as_mut_slice())?;

    let raw_fd = tcp.as_raw_fd();

    let tcp_info = get_tcp_info(raw_fd)?;
    println!("tcp_info: {:#?}", tcp_info);

    Ok(())
}
