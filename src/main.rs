use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, BufRead};
use std::fs;
use std::path::Path;
use std::env::args;
use std::thread;

const ver: &str = "0.1";

fn main() -> std::io::Result<()>{
    let path = std::env::args().nth(1).expect("Direcotry path not set");
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Started on {}", listener.local_addr()?);
    
    if !Path::new(&path).is_dir(){
        println!("{} doesnt exist or isnt a dir", path);
    }

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                let pathClone = path.clone();
                thread::spawn(move || response(stream, pathClone.to_string()));
            }
            Err(e) => {
                eprintln!("Accept error {}", e);
            }
        }
    }
    Ok(())
}

fn response(mut stream: TcpStream, path: String) {
    let addr = stream.peer_addr().unwrap();

    let readerBuf = io::BufReader::new(&mut stream);
    let requestLine = readerBuf.lines().next().unwrap().unwrap();
    let request: Vec<&str> = requestLine.split_whitespace().collect();

    println!("Connection from {}, requested URL {}",addr,request[1]);

    if request[1] == "/" {
        if !Path::new(&format!("{}/index.html", path)).is_file() || !Path::new(&path).is_dir(){
            let page = format!("If you see this then the index.html is missing");
            stream.write_all(format!("HTTP/1.1 200 OK\r\\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",page.len(),page).as_bytes());
        }
        else {
            let content = fs::read(format!("{}/index.html", path.to_owned())).expect("Failed to open");
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",content.len()).as_bytes());
            stream.write_all(&content);
        }
    }
    else if request[1] == "/server/fox" {
        let page = format!("<style>body {{ background-image: url('https://blob.gifcities.org/gifcities/VCBC4R6HN2RHKQARM75JHXG3AT2KQMDU.gif'); color: white }}</style>
        <p>Oh! Hey there!! You managed to find an easter egg! This easter egg page contains random fox image. (Also, if you're wondering, this easter egg exists in EVRY copy of FWebServer.)</p>
        <img id='foxImg' width=500></img>
        <p><a href='/'>Go to the root page</a> | <a href='/server/fox'>Reload this page</a></p>
        <p>FWebServer/{}, created by Foxy3049 in 2026</p>
        <script>fetch('https://randomfox.ca/floof/').then(response => response.json()).then(data => document.getElementById('foxImg').src = data.image); </script>", ver);
        stream.write_all(format!("HTTP/1.1 200 OK\r\\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",page.len(),page).as_bytes());
    }
    else{
        if Path::new(&(format!("{}{}", path.to_owned(), request[1]))).exists(){
            let content = fs::read(format!("{}{}", path.to_owned(), request[1])).expect("Failed to open");
            stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",content.len()).as_bytes());
            stream.write_all(&content);
        }
        else {
            let page = format!("<center><h1>404: Not Found</h1>
                                <p>Requested URL {} was not found on this server.</p>
                                <a href='/'>Go to the root page</a>
                                <hr>
                                FWebServer/{}</center>",request[1],ver);
            stream.write_all(format!("HTTP/1.1 404 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",page.len(),page).as_bytes());
        }
    }
}
