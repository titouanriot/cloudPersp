/*use reqwest::Url;
use http::Uri;
use std::borrow::Cow;*/
use std::{
    fs::{self, OpenOptions, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, SocketAddr}, time::SystemTime
};
use tiny_http::{Header, Response, Request};
use chrono::prelude::*;
  
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// telnet 127.0.0.1 8080

fn handle_connection(mut stream: TcpStream) {
    println!("Incoming connection from: {:?} \n", stream.peer_addr());
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = "".to_string();
    let mut read_buf = Vec::new();
    buf_reader.read_to_end(&mut read_buf);
    let mut header_line = String::from_utf8(read_buf);
    // loop {
    //     buf_reader.read_line(&mut header_line);

    //     // The final line is just /r/n
    //     if header_line.len() == 2 {
    //         break
    //     }
    //     header_line = "".to_string();
    // }

    // This buffer would need to be whatever size Content-Length reports
    // buf_reader.read_exact(&mut read_buf);

    // let body = String::from_utf8(read_buf.to_vec());
    // println!("BODY: {}", body.unwrap());             TD
    println!("{}", header_line.unwrap());
    
 //////////TEEEEEEEESSSSSSSSTTTTTTTTTTTTTT
    
    // let mut buf_reader = BufReader::new(&mut stream);


    // let mut buf = [0u8; 32];
    // let read = buf_reader.read(&mut buf);
    // println!("Read : {:?}", read);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request: {:#?} \n", http_request);
    // let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    // let contents = fs::read_to_string("hello.html").unwrap();
    // let length = contents.len();
    // let response = format!("{status_line}Content-Length: {length}\n{contents}");
    // println!("{}", response);
    
    // stream.write_all(response.as_bytes()).unwrap();
}

/*fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}*/

fn open_connection() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];

    if let Ok(stream) = TcpStream::connect(&addrs[..]) {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }
}

/*fn get_params_by_url(req: String) -> Vec<(String, String)> {
    let uri = req.parse::<Uri>().unwrap();

    let tmp = Url::parse(&uri.to_string()).unwrap();
    let params = tmp.query_pairs();

    let vec_params = params.collect::<Vec<(Cow<'_, str>, Cow<'_, str>)>>();

    let mut res = vec![];
    for vp in vec_params {
        res.push((String::from(vp.0), String::from(vp.1)));
    }

    return res;
}*/


fn write_incoming_ip(request : Option<&SocketAddr>){
    let fp = "./beacon.txt";
    let does_exist = std::path::Path::new(fp).exists();
    if !does_exist {
        File::create(fp).unwrap();
    }  
    let mut ip = String::from("Unknown IP");
    match request{
        Some(res) => {
            ip = String::from(res.to_string());
            let temp_ip = String::from(res.to_string());
            let split = temp_ip.split(":");
            let mut compteur = true;
            for l in split{
                if compteur{
                    ip = String::from(l);
                    compteur = false;
                }
            }
            println!("{}", ip);
        },
        None => ()
    }
    let file = File::open(fp).unwrap();
    let reader = BufReader::new(file);
    let mut does_exist = false;
    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.contains(&ip) {
                    does_exist = true;
                }
            },
            Err(_) => {
                println!("error reading lines in file");
            }
        }
    }
    if !does_exist{
        println!("!does_exist");
        let mut file_ref = OpenOptions::new().append(true).open(fp).expect("Unable to open file"); 
        file_ref.write_all(ip.as_bytes()).expect("write failed");
        file_ref.write_all(" - active \n".as_bytes()).expect("write failed");
    }
}

fn write_logs(request : Option<&SocketAddr>){
    let fp = "./logs.txt";
    let does_exist = std::path::Path::new(fp).exists();
    if !does_exist {
        File::create(fp).unwrap();
    }
    let mut file_ref = OpenOptions::new().append(true).open(fp).expect("Unable to open file");   
    file_ref.write_all("Incoming connection from: ".as_bytes()).expect("write failed");
    let mut ip = String::from("Unknown IP");
    match request{
        Some(res) => {
            ip = String::from(res.to_string());
        },
        None => ()
    }
    file_ref.write_all(ip.as_bytes()).expect("write failed");
    file_ref.write_all(" at : ".as_bytes()).expect("write failed");
    let date_as_string = Utc::now().to_string();
    file_ref.write_all(date_as_string.as_bytes()).expect("write failed");
    file_ref.write_all("\n".as_bytes()).expect("write failed");
    println!("Log appended successfully"); 
}

#[tokio::main]
async fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
    //println!("listening started, ready to accept");
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     handle_connection(stream);
    // }
    let server = tiny_http::Server::http("0.0.0.0:8082").unwrap();
    for mut request in server.incoming_requests() {
        println!("Incoming connection from: {:?} \n", request.remote_addr());
        write_incoming_ip(request.remote_addr());
        write_logs(request.remote_addr());
        // println!("received request! method: {:?}, url: {:?}, headers: {:?}",
        //     request.method(),
        //     request.url(),
        //     request.headers()
        // );
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        println!("{}", content);
        let response = Response::from_string("Response strin blabla");
        request.respond(response).unwrap();
    }
    /*let params = [("foo", "kjj"), ("baz", "quux")];
    let resp = Client::new()
        .post("http://127.0.0.1:5500/test.html?foo=1&bar=2")
        .form(&params)
        .body("the exact body that is sent")
        .send()
        .await;

    let request = String::from("http://127.0.0.1:5500/test.html?foo=value1&bar=value2");

    let resp = get_params_by_url(request);

    dbg!(resp);*/
}