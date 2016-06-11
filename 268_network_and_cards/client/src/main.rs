use std::io::prelude::*;
use std::net::{TcpStream,SocketAddrV4};
use std::env;
extern crate clap;
use clap::{Arg, App, SubCommand};
use std::thread::sleep;
use std::time::Duration;

fn main() {
   
	//setting arguments 
    let matches = App::new("client for 268 dailyprogramming challange")
				  .version("1.0")
				  .author("Pedro Ramos <pedrolorgaramos@gmail.com>")
				  .about("client side of 268 dailyprogramming challange")
				  .arg(Arg::with_name("ip")
					   .short("i")
					   .long("ip")
					   .value_name("IP")
					   .help("Sets the server ip address. Default is 127.0.0.1")
					   .takes_value(true))
                  .arg(Arg::with_name("port")
                       .short("p")
                       .long("port")
                       .value_name("PORT")
                       .takes_value(true)
                       .help("server's port. Default is 34254"))
				  .arg(Arg::with_name("v")
					   .short("v")
					   .multiple(true)
					   .help("Sets the level of verbosity"))
                  .arg(Arg::with_name("debug")
                       .short("d")
                       .long("debug")
                       .help("prints debug information"))
                  .arg(Arg::with_name("username")
                        .short("u")
                        .long("username")
                        .value_name("USERNAME")
                        .help("login username")
                        .takes_value(true))
				  .get_matches();
    
    //starting connection
    let mut buffer:String = String::new();
    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("34254");
    let mut stream = TcpStream::connect((ip, port.parse::<u16>().expect("Port number is invalid"))).expect("Failed to connect");



    //registation of username
    stream.write("USERNAME".as_bytes());
    stream.write(matches.value_of("username").unwrap_or("anonymous").as_bytes());

    stream.read_to_string(&mut buffer);
    println!("{}", buffer);
    match buffer.as_str(){
        "OK" => println!("user registation was sucessfull"),
        "NOK" => {println!("user already being used");panic!()},
        _ => {println!("what!?");unreachable!()},
    }
    
    loop{
        sleep(Duration::from_secs(10));
        match stream.write("bumbum".as_bytes()){
            Ok(_) => (),
            Err(_) => println!("write timeout"),
        }
    }
}




