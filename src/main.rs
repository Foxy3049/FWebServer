use std::net::{TcpListener, TcpStream};
use std::io::{self, Write, BufRead};
use std::fs;
use std::path::Path;
use std::thread;

const path: &str = "/home/foxy3049/.fwebserver";
const ver: &str = "0.1";

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Started on {}", listener.local_addr()?);

    if !Path::new(path).is_dir(){
        fs::create_dir("/home/foxy3049/.fwebserver");
    }
    
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                thread::spawn(|| response(stream));
            }
            Err(e) => {
                eprintln!("Accept error {}", e);
            }
        }
    }
    Ok(())
}

fn response(mut stream: TcpStream) {
    let addr = stream.peer_addr().unwrap();

    let readerBuf = io::BufReader::new(&mut stream);
    let requestLine = readerBuf.lines().next().unwrap().unwrap();
    let request: Vec<&str> = requestLine.split_whitespace().collect();

    println!("Connection from {}, requested URL {}",addr,request[1]);

    if request[1] == "/" {
        let content = fs::read(path.to_owned() + "/index.html").expect("Failed to open");
        stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",content.len()).as_bytes());
        stream.write_all(&content);
    }
    else if request[1] == "/server/fox" {
        let page = format!("<style>body {{ background-image: url('https://blob.gifcities.org/gifcities/VCBC4R6HN2RHKQARM75JHXG3AT2KQMDU.gif'); color: white }}</style>
        <p>Oh! Hey there!! You have managed to find an easter egg! This easter egg page contains random fox image. (Also, if you're wondering, this easter egg is in EVRY copy of FWebServer.)</p>
        <img id='foxImg' width=500></img>
        <p><a href='/'>Go to the root page</a> | <a href='/server/fox'>Reload this page</a></p>
        <p>FWebServer/{}, created by Foxy3049 in 2026</p>
        <script>fetch('https://randomfox.ca/floof/').then(response => response.json()).then(data => document.getElementById('foxImg').src = data.image); </script>", ver);
        stream.write_all(format!("HTTP/1.1 200 OK\r\\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",page.len(),page).as_bytes());
    }
    else{
        if Path::new(&(path.to_owned() + request[1])).exists(){
            let content = fs::read(path.to_owned() + request[1]).expect("Failed to open");
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
