use std::io::{stdin};
use std::process::exit;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::io::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio::sync::mpsc::channel;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    spawn(async move {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap_or_default();

            if input.contains("exit") || input.contains("quit") {
                println!("\nProgram exit... OK\n");

                exit(0);
            }
        }
    });

    let stream_list: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));

    let (message_sender, message_receiver) = channel::<(String, usize)>(100);

    let message_sender_arc = Arc::new(Mutex::new(message_sender));

    let message_receiver_arc = Arc::new(Mutex::new(message_receiver));

    let stream_list_clone1 = Arc::clone(&stream_list);

    let message_sender_arc_clone = Arc::clone(&message_sender_arc);

    spawn(async move {
        loop {
            for (i, stream_i) in stream_list_clone1.lock().await.iter_mut().enumerate() {
                let mut buf = [0u8; 32];
                if stream_i.lock().await.try_read(&mut buf).unwrap_or_default() > 0 {
                    println!("{}", from_utf8(&buf).unwrap_or_default());
                    message_sender_arc_clone.lock().await.try_send((from_utf8(&buf).unwrap_or_default().to_string(), i)).unwrap_or_default();
                }
            }
        }
    });

    let stream_list_clone2 = Arc::clone(&stream_list);

    let message_receiver_arc_clone = Arc::clone(&message_receiver_arc);

    spawn(async move {
        loop {
            if let Ok(msg) = message_receiver_arc_clone.lock().await.try_recv() {
                for (i, stream_i) in stream_list_clone2.lock().await.iter_mut().enumerate() {
                    if i != msg.1 && !msg.0.contains("exit") && !msg.0.contains("quit") {
                        stream_i.lock().await.try_write(msg.0.as_bytes()).unwrap_or_default();
                    }
                }
            }
        }
    });

    let stream_list_clone3 = Arc::clone(&stream_list);

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let stream_arc = Arc::new(Mutex::new(stream));
    
            stream_list_clone3.lock().await.push(stream_arc);
        }
    }
}
