/*use reqwest::Url;
use http::Uri;
use std::borrow::Cow;*/
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, SocketAddr}
};
use http::Method;
use tiny_http::{Header, Response, Request};
use serde::{Serialize, Deserialize};
use std::thread;
  
// https://doc.rust-lang.org/book/ch20-01-single-threaded.html
// telnet 127.0.0.1 8080

fn handle_connection(mut stream: TcpStream) {
    println!("Incoming connection from: {:?} \n", stream.peer_addr());



    //////////TEEEEEEEESSSSSSSSTTTTTTTTTTTTTT
    let mut buf_reader = BufReader::new(&mut stream);

    let mut request_line = "".to_string();
    // buf_reader.read_line(&mut request_line);


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


#[derive(Serialize, Deserialize, Debug, Clone)]
enum OrdreType {
    Commande,
    Fichier,
    Endormir,
    Autre
}

#[derive(Serialize, Deserialize, Debug)]
struct Ordre {
    ordre: OrdreType,
    arguments: Vec<String>,
}






async fn handle_post_request(server: & tiny_http::Server) -> () {

    let request = server.recv();

    match request {
        Ok(mut rq) => {

            if *rq.method() == tiny_http::Method::Post {

                let mut content = String::new();
                rq.as_reader().read_to_string(&mut content).unwrap();

                println!("{}", content);
            
                let response = Response::from_string("Recu requete POST\n");
                rq.respond(response);
            }
        },
        Err(e) => { println!("error: {}", e);  }
    };
}

async fn handle_file_post_request(server: & tiny_http::Server, filename: &str) -> () {

    let request = server.recv();

    match request {
        Ok(mut rq) => {

            if *rq.method() == tiny_http::Method::Post {
                
                let file = std::fs::File::open(filename).unwrap();

                let mut response = Response::from_file(file);
                let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"multipart/form-data"[..]).unwrap();
                response.add_header(header);

                // let mut content = String::new();
                // rq.as_reader().read_to_string(&mut content).unwrap();

                // println!("{}", content);
            
                // let response = Response::from_string("Recu requete POST\n");
                rq.respond(response);
            }
        },
        Err(e) => { println!("error: {}", e);  }
    };
}


async fn send_ordre(server: & tiny_http::Server, ordre: OrdreType, arguments: Vec<String>) -> () {
    let request = server.recv();
    match request {
        Ok(rq) => {

            if *rq.method() == tiny_http::Method::Get {

                let bod = Ordre { ordre: ordre.clone(), arguments: arguments.clone() };

                let response = Response::from_string(serde_json::to_string(&bod).unwrap());
                rq.respond(response);


                match ordre {
                    OrdreType::Commande => {
                        handle_post_request(&server).await;
                    },
                    OrdreType::Fichier => {
                        let filename = arguments[0].as_str();
                        handle_file_post_request(&server, filename).await;
                    },
                    _ => ()
                }


            }

        },
        Err(e) => { println!("error: {}", e);  }
    };

}






#[tokio::main]
async fn main() {
    let server = tiny_http::Server::http("0.0.0.0:8082").unwrap();
    // let server = std::sync::Arc::new(server);

    //////////////////////// gère en permanence les requetes post  (utilise le multithreading, a voir si utile eventuellement)
    // let server_reception = server.clone();
    // let mut reception = thread::spawn(move || {

    //     for mut request in server_reception.incoming_requests() {
    //         // println!("received request! method: {:?}, url: {:?}, headers: {:?}",
    //         //     request.method(),
    //         //     request.url(),
    //         //     request.headers()
    //         // );

    //         if *request.method() == tiny_http::Method::Post {

    //             let mut content = String::new();
    //             request.as_reader().read_to_string(&mut content).unwrap();

    //             println!("{}", content);
            
    //             let response = Response::from_string("Recu requete POST\n");
    //             request.respond(response);
    //         }
    //         // else if *request.method() == tiny_http::Method::Get {



    //         //     let bod = Ordre { ordre: OrdreType::Commande, arguments: vec![String::from("ls"), String::from("-l")] };


    //         //     let response = Response::from_string(serde_json::to_string(&bod).unwrap());
    //         //     // let response = Response::from_string("Recu requete GET\n");
    //         //     request.respond(response);
    //         // }
    //     }
    // });
    ////////////////////////




    //////////////////////// Envoie une commande ls pour l'example
    send_ordre(&server, OrdreType::Commande, vec![String::from("ls"), String::from("-l")]).await;
    //////////////////////// envoie un echo pour l'exemple
    send_ordre(&server, OrdreType::Commande, vec![String::from("echo"), String::from("titouan")]).await;
    //////////////////////// envoie un fichier pour l'exemple
    send_ordre(&server, OrdreType::Fichier, vec![String::from("texte.txt")]).await;


    // reception.join().unwrap();

}