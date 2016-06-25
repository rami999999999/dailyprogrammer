use std::io::prelude::*;
use std::env;
extern crate clap;
extern crate time;
use time::Tm;
use clap::{Arg, App, SubCommand};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::string::String;
use std::sync::mpsc;
use std::collections::HashMap;


#[derive(PartialEq)]
enum User{
    username(String),
    private,
    None
}

///function to get e free thread id
fn get_id( input: &[User]) -> i32 {
    let mut i=0;
    let mut found=0;
    for j in input{
        if *j != User::None {break;found=1;}
        else {i+=1;}
    } 
    if found == 1 {i}
    else {-1}
}

///Executed function when USR is recieved
fn usr_command(mut stream: &TcpStream, mut lbuffer: &mut [u8], v:bool,id:i32){
    
    if v {println!("thread-{} | executing usr",id)};
    let mut read_counter:usize;
    
    read_counter = match stream.read(&mut lbuffer) {
        Ok(n) => n,
        Err(_) => {println!("thread-{} | connection failed onr read",id); panic!(); 0 as usize},
    };

    if v {println!("thread-{} | incoming:{}",id, std::str::from_utf8(&lbuffer[..read_counter]).unwrap())};
    
    let username = String::from_utf8_lossy(&lbuffer[..read_counter]).into_owned();    
    
    stream.write("OK".as_bytes());
    
    if v {println!("thread-{} | My username is {}",id,username)}
}

///function that is started for each recieved connection
fn handle_client(mut stream: TcpStream, v:bool, id:i32) {
    
    //short buffer
    let mut buffer: [u8;3] = [0,0,0];

    //long buffer
    let mut lbuffer: [u8;528] = [0;528];
    
    let mut read_counter:usize;
    let mut last_con: Tm = time::now();
    
    //main loop
    loop {       
        read_counter = match stream.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => {println!("thread-{} | connection failed onr read",id); panic!(); 0 as usize},
        };

        if v {println!("thread-{} | incoming:{}",id,std::str::from_utf8(&buffer[..read_counter]).unwrap());}

        match std::str::from_utf8(&buffer[..read_counter]).unwrap() {
            "..." => {last_con=time::now(); if v {println!("thread-{} | I'm alive",id)}},
            "USR" => {last_con=time::now(); usr_command(&mut stream, &mut lbuffer,v,id);},
            _ => unreachable!(),
        }
    }
}


///Main function of the server
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
					   .help("Sets the level of verbosity"))
                  .arg(Arg::with_name("debug")
                       .short("d")
                       .long("debug")
                       .help("prints debug information"))
				  .get_matches();

    //verbose flag
    let mut v:bool = false;
    let port = matches.value_of("port").unwrap_or("34254");
    let mut id = 0;
    
    let  mut p_list : Vec<User> = Vec::with_capacity(500 as usize);
    for i in 0..500 {
        p_list.push(User::None);
    }

    let listener = TcpListener::bind(("127.0.0.1", port.parse::<u16>().expect("Port number is invalid"))).expect("Failed to connect");

    if matches.is_present("v"){
        v=true;
    }
    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                
                //Getting id
                id = get_id(&p_list);
                if id == -1 {id = p_list.len() as i32 + 1; p_list.push(User::None);}

                thread::Builder::new().name(format!("thread-{}",id).to_string()).spawn(move|| {
                    if v {println!("starting thread-{}",id)}
                    handle_client(stream, v, id)
                });
                //TODO:getusername
                           

            }
            Err(e) => { /* connection failed */ }
        }
    }

}



