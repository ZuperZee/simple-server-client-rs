use std::{
    fs,
    io::Read,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
};

fn main() {
    let mut stream =
        TcpStream::connect(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)).unwrap();

    let mut buf = [0; 256];
    let n = stream.read(&mut buf).unwrap();

    let data = &buf[..n];
    let s = String::from_utf8_lossy(data);

    println!("{:?}", data);
    println!("{:?}", s);

    fs::write("./out.json", data).unwrap();
}
