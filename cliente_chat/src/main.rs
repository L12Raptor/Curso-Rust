use std::{
    io::{stdin, Read, Write, ErrorKind},
    net::TcpStream,
    str::from_utf8,
    sync::mpsc::{channel, TryRecvError},
    thread::spawn,
};

fn main() {
    let mut server_address = String::new();

    println!("Introduce the server address and port (<address>:<port>):");

    stdin().read_line(&mut server_address).unwrap_or_default();

    let mut stream = TcpStream::connect(server_address.trim()).unwrap();

    stream.set_nonblocking(true).unwrap();

    let (sender, receiver) = channel::<String>();

    spawn(move || loop {
        let mut buf = [0u8; 32];

        match stream.read(&mut buf) {
            Ok(_) => println!("{}", from_utf8(&buf).unwrap_or_default()),
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("\nServer disconnected!\n");

                break;
            },
        }

        match receiver.try_recv() {
            Ok(msg) => {
                stream.write(msg.as_bytes()).unwrap_or_default();
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break,
        }
    });

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap_or_default();

        sender.send(input.trim_end().to_string()).unwrap_or_default();

        if input.contains("exit") || input.contains("quit") {
            println!("\nGood bye!\n");
            break;
        }
    }
}
