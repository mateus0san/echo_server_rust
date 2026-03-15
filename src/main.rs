use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};
fn main() {
    let server = TcpListener::bind("0.0.0.0:8080").unwrap();

    for mut stream in server.incoming().flatten() {
        thread::spawn(move || {
            loop {
                let mut buf = [0; 10];
                Read::by_ref(&mut stream)
                    .take(10)
                    .read_exact(&mut buf)
                    .unwrap();
                let _ = stream.write(&buf);
            }
        });
    }
}
