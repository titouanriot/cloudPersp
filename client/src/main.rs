use std::io::prelude::*;
use std::net::TcpStream;
use std::{thread, time};
use std::process::Command;
use std::io::Write;



// let output = Command::new("echo")
// .arg("hello world")
// .status()
// .expect("failed to execute");

// let list_dir = Command::new("ls")
// // command arguments
// .args(["-l", "-a", "-R", "-t"])
// // directory we want to display the files
// .current_dir("src/")
// .status()
// .expect("ls command failed to start");

// let change_permissions = Command::new("chmod")
// .args(["-R", "777", "test/"])
// .status()
// .expect("chmod command failed to start");


//Exemple utilisation de la commande
// let vecteur = vec!["-a".to_string(), "-l".to_string()];
// exec_commande_shell(String::from("ls"), vecteur);
fn exec_commande_shell(command : String, args : Vec<String>) -> std::io::Result<()> {
    let mut base_command = Command::new(command);
    if args.len() > 0 {
        for i in 0..args.len() {
            base_command.arg(args[i].clone());
        }
    }
    //base_command.output()
    //.expect("failed to excute shell command");
    base_command.output().expect("error executing command");
    base_command.status();
    base_command.output();
    //println!("{}", base_command);
    //println!("{}", String::from_utf8_lossy(&base_command.stdout()));
    //io::stdout().write_all(&base_command.stdout()).unwrap();
    return Ok(());      
}

fn sending_request() -> std::io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    let time = time::Duration::from_millis(1000);
    thread::sleep(time);
    return Ok(());
}


fn main() {
    let boucle : bool = true;
    while boucle {
        sending_request();
    }
}
