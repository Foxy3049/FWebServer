use std::fs;
use std::io::{self, BufRead, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread;

const VER: &str = "0.2";

fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1).expect("Directory path not set");

    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Started on {}", listener.local_addr()?);

    if !Path::new(&path).is_dir() {
        println!("{} doesnt exist or isnt a dir", path);
        std::process::exit(1);
    }

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let path_clone = path.clone();
                thread::spawn(move || response(stream, path_clone));
            }
            Err(e) => {
                eprintln!("Accept error {}", e);
            }
        }
    }
    Ok(())
}

fn response(mut stream: TcpStream, path: String) {
    let reader_buf = io::BufReader::new(&mut stream);

    let request_line = reader_buf.lines().next().unwrap().unwrap();
    let request: Vec<&str> = request_line.split_whitespace().collect();

    let base_dir = match fs::canonicalize(&path) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    let relative_path = if request[1] == "/" {
        "index.html"
    } else {
        request[1].trim_start_matches('/')
    };
    let target_path = base_dir.join(relative_path);

    println!(
        "Connection from {}, requested URL {}",
        stream.peer_addr().unwrap(),
        request[1]
    );

    match fs::canonicalize(target_path) {
        Ok(full_path) if full_path.starts_with(&base_dir) && full_path.is_file() => {
            if let Ok(content) = fs::read(full_path) {
                let header = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                    content.len()
                );
                let _ = stream.write_all(header.as_bytes());
                let _ = stream.write_all(&content);
            }
        }
        _ => {
            let page = format!(
                "<center><h1>404: Not Found</h1>
                <p>Requested URL '{}' was not found on this server.</p>
                <a href='/'>Go to the root page</a>
                <hr>
                FWebServer/{}</center>",
                request[1], VER
            );
            let response = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                page.len(),
                page
            );
            let _ = stream.write_all(response.as_bytes());
        }
    }
}
