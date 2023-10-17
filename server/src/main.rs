use std::{net::{TcpListener, SocketAddr, IpAddr, Ipv4Addr, TcpStream}, io::Write, time::{SystemTime, UNIX_EPOCH}};

fn main() {
    let listener = TcpListener::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000)).unwrap();

    let mut counter = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream, &mut counter);
            }
            Err(e) => {
                println!("{e:?}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, counter: &mut u64) {
    let ms_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let write_str = format!("{{\"counter\": {counter}, \"ms_since_epoch\": {ms_since_epoch}}}");
    let write_buf = write_str.as_bytes();
    stream.write_all(write_buf).unwrap();
    *counter += 1;
}
