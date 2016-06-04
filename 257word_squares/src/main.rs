use std::io::BufReader;
use std::io;
use std::env;

struct word{
text:String,
n: i32
}



fn main() {

//collecting arguments
let args: Vec<_> = env::args().collect::<Vec<_>>();

if args.len() < 2 {
	println!("missing arguments");
	//exit;
	}

assert!(args[0].parse::<i32>().is_ok(), "ERROR");
let size= args[0].parse::<i32>().unwrap();
if size*size != args[1].chars().collect::<Vec<_>>().len() as i32 {
        println!("Size of the grid does not match the number of letters")
        //exit:
        }

//TODO open file and get all words with 'size' size

//TODO for each word there must be a pair of each char except for now. Find all the worlds that meet this critirira

}
