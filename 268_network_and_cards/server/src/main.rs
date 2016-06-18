use std::io::prelude::*;
use std::env;
extern crate clap;
extern crate time;
use time::Tm;
use clap::{Arg, App, SubCommand};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::String;


fn handle_client(mut stream: TcpStream) {
    //short buffer
    let mut buffer: [u8;3] = [0,0,0];
    //long buffer
    let mut lbuffer: [u8;528] = [0;528];
    let mut read_counter:usize;
    let mut last_con: Tm = time::now();
    
    //reading command
    stream.read(&mut buffer);
    match std::str::from_utf8(&buffer).unwrap() {
        "USR" => println!("USR recieved"),
        _ => unreachable!(),
    }
    

    read_counter = match stream.read(&mut lbuffer) {
        Ok(n) => n,
        Err(_) => {println!("connection failed onr read"); panic!(); 0 as usize},
    };

    println!("line25:{}",std::str::from_utf8(&lbuffer[..read_counter]).unwrap());
    let username = buffer.clone();    
    stream.write("OK".as_bytes()); 
    loop {
        
        read_counter = match stream.read(&mut lbuffer) {
            Ok(n) => n,
            Err(_) => {println!("connection failed onr read"); panic!(); 0 as usize},
        };


        println!("{}",std::str::from_utf8(&lbuffer[..read_counter]).unwrap());

        match std::str::from_utf8(&lbuffer[..read_counter]).unwrap() {
            "bumbum" => {last_con=time::now();println!("I'm alive")},
            _ => unreachable!(),
        }
    }
}



fn main() {
   
	 
    let matches = App::new("server for 268 dailyprogramming challange")
				  .version("1.0")
				  .author("Pedro Ramos <pedrolorgaramos@gmail.com>")
				  .about("server side of 268 dailyprogramming challange")
                  .arg(Arg::with_name("port")
                       .short("p")
                       .long("port")
                       .value_name("PORT")
                       .takes_value(true)
                       .help("server's port"))
				  .arg(Arg::with_name("v")
					   .short("v")
					   .multiple(true)
					   .help("Sets the level of verbosity"))
                  .arg(Arg::with_name("debug")
                       .short("d")
                       .long("debug")
                       .help("prints debug information"))
				  .get_matches();



    //starting connection
    let port = matches.value_of("port").unwrap_or("34254");

    let listener = TcpListener::bind(("127.0.0.1", port.parse::<u16>().expect("Port number is invalid"))).expect("Failed to connect");

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }

}



