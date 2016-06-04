use std::{env,fs};
use std::io::{BufRead, BufReader};

fn main() {
    //reading files from arguments
    //only argument is file_path
    let file_dir: String = env::args().nth(1).unwrap_or(String::from("input/input1.txt"));
    println!("{}", file_dir);

    //now opening file
    let f = fs::File::open(file_dir).expect("could not open file");
    
    let mut reader = BufReader::new(f);  
    
    //current level of identation
    let mut current = 0 ;

    //original number of > or . present
    let mut ident_counter = 0;
    
    for line in reader.lines(){
        let line: String = line.expect("Could not read line");
        
        if line.chars().nth(0).is_none() == true {continue; }
        
        if line.chars().nth(0).unwrap() == '·' || line.chars().nth(0).unwrap() == '»'{
            for i in line.chars(){
                if i == '·' || i == '»'{
                    ident_counter+=1;
                }
                
            }
        }
    
        //println!("LINE::{}", line);
        //println!("counter::{}",ident_counter);
        let indice = line.char_indices().nth(ident_counter);
        if indice == None { println!("");}
        else {
            let indice = indice.unwrap().0;
            let no_idented:&str = &line[indice..];

            //println!("PARSED::{}", no_idented);
            if no_idented.len() != 0 {
                match no_idented.split_whitespace().nth(0).unwrap(){
                   "IF" | "FOR" => current+=1,
                   "ENDIF" | "NEXT" => current-=1,
                    _ => (),
                }
            }
            
            for n in 0..current {
                print!("....");
            }
            println!("{}", no_idented);
        }
        ident_counter=0;
    }   

    

}



