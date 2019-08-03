use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn handle_client(mut stream: TcpStream, data: &[u8]) -> std::io::Result<usize> {
    loop {
        stream.write(data)?;
        stream.flush();
    };
    Ok(0)
}

fn main() -> std::io::Result<()>{


    let listener = TcpListener::bind("127.0.0.1:1111")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        std::thread::spawn(|| {
            let data = "1".repeat(1024);
            handle_client(stream?, data.as_bytes())
        });
    }
    Ok(())
}
