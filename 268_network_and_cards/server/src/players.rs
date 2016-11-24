use std::sync::mpsc;

pub type User =  Option<Player>;


pub struct Player{
    //send to a thread
    pub username: String,
    //send to a thread
    //tx: mpsc::Sender<[u8;100]>,
    //recieve from a thread
    pub Prx: mpsc::Receiver<[u8;100]>,
    pub Ptx: mpsc::Sender<[u8;100]>,
}




///function to get e free thread id
pub fn get_id( input: &[Option<Player>]) -> i32 {
    let mut i=0;
    let mut found=0;
    for j in input{
        if (*j).is_none() {break;found=1;}
        else {i+=1;}
    }
    if found == 1 {i}
    else {-1}
}
