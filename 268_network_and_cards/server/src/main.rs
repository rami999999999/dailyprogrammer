use std::io::prelude::*;
use std::net::TcpStream;
use std::env;
extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
   
	 
    let matches = App::new("server for 268 dailyprogramming challange")
				  .version("1.0")
				  .author("Pedro Ramos <pedrolorgaramos@gmail.com>")
				  .about("server side of 268 dailyprogramming challange")
				  .arg(Arg::with_name("server ip")
					   .short("i")
					   .long("ip")
					   .value_name("IP")
					   .help("Sets the server ip address")
					   .takes_value(true)
                       .required(true))
                  .arg(Arg::with_name("server port")
                       .short("p")
                       .long("port")
                       .value_name("PORT")
                       .takes_value(true)
                       .help("server's port")
                       .required(true))
				  .arg(Arg::with_name("v")
					   .short("v")
					   .multiple(true)
					   .help("Sets the level of verbosity"))
                  .arg(Arg::with_name("debug")
                       .short("d")
                       .long("debug")
                       .help("prints debug information"))
				  .get_matches();

}



