use std::{
    fs::{OpenOptions, File},
    io::{prelude::*, BufReader},
    net::{SocketAddr}
};
use tiny_http::{Response};
use chrono::prelude::*;
  
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// telnet 127.0.0.1 8080

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
    let server = tiny_http::Server::http("0.0.0.0:8082").unwrap();
    for mut request in server.incoming_requests() {
        println!("Incoming connection from: {:?} \n", request.remote_addr());
        write_incoming_ip(request.remote_addr());
        write_logs(request.remote_addr());
        let mut content = String::new();
        request.as_reader().read_to_string(&mut content).unwrap();
        println!("{}", content);
        let response = Response::from_string("Response strin blabla");
        request.respond(response).unwrap();
    }
}