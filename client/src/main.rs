use std::process::{Command};
use reqwest;
//use std::io::{BufWriter, Write};
use std::str;

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
    .body(result_command)   //;.strip_prefix("\n\r") .stdout    
    .send()
    .await;
    //format!("{:?}", result_command.stdout)
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
    // let boucle : bool = true;
    // while boucle {
    //     sending_request();
    // }
    //sending_request();
    let vecteur = vec!["-a".to_string(), "-l".to_string()];
    let result_command = exec_commande_shell(String::from("ls"), vecteur);
    match result_command{
        Ok(mut v) => {
            let mut string_result = String::from(""); //str::from_utf8(&v.output().stdout).unwrap();
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

//String::from(format!("{:?}", line.stdout))


    // let file = fs::File::open("much_beauty.png")?;
    // let client = reqwest::Client::new();
    // let res = client.post("http://httpbin.org/post")
    // .headers(construct_headers())
    // .body(file)
    // .send()?;
    // let result = reqwest::get("http://127.0.0.1:8082")
    // .unwrap()
    // .text()
    // .await;

    // let  pre_string_result = format!("{:?}", result_command);
    // println!("{}", pre_string_result);
    // let string_result = pre_string_result.replace("\n", "");
    // println!("{}", pre_string_result.eq(&string_result));
    //let mut json_result = json::stringify(string_result);
    // let json: serde_json::Value =
    //     serde_json::from_str(the_file).expect("JSON was not well-formatted");
    //println!("{}", string_result);
    //let file = File::open("temp.txt")?;
// fn sending_request() -> std::io::Result<()>{
//     let mut stream = TcpStream::connect("127.0.0.1:7878")?;
//     stream.write(&[1])?;
//     stream.read(&mut [0; 128])?;
//     let time = time::Duration::from_millis(1000);
//     thread::sleep(time);
//     return Ok(());
// }

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
