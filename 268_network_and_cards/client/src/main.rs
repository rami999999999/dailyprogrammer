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
    let mut rbuffer: [u8;528] = [0;528];
    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("34254");
    let mut stream = TcpStream::connect((ip, port.parse::<u16>().expect("Port number is invalid"))).expect("Failed to connect");
    let mut buffer: String = String::new();
    let mut read_counter:usize;
    let mut v = false;

    if matches.is_present("v"){
        v = true;
    }

    //registation of username
    if v {println!("start user registation")};
    stream.write("USR".as_bytes());
    stream.write(matches.value_of("username").unwrap_or("anonymous").as_bytes());

    read_counter = match stream.read(&mut rbuffer) {
        Ok(n) => n,
        Err(_) => {println!("connection failed onr read"); panic!(); 0 as usize},
    };
    if v {println!("{:?}", std::str::from_utf8(&rbuffer[..2]).unwrap());}
    match std::str::from_utf8(&rbuffer[..2]).unwrap(){
        "OK" => if v {println!("user registation was sucessfull")},
        "NOK" => {println!("user already being used");panic!()},
        _ => {if v {println!("what!?. Was expecting OK or NOK")};unreachable!()},
    }
    
    loop{
        sleep(Duration::from_secs(10));
        match stream.write("...".as_bytes()){
            Ok(_) => (),
            Err(_) => println!("write timeout"),
        }
    }
}




