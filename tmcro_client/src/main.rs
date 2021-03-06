use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::{TcpListener, TcpStream};
use json::JsonValue;
use log::error;
use memory::*;

mod session;
mod inventory;
mod memory;

fn handle_json(){

}

fn read_message(buffer: &mut BufReader<TcpStream>) -> json::Result<JsonValue>{
    let mut vecmsg = Vec::new();
    buffer.read_until(10,&mut vecmsg);
    let mut msg=std::str::from_utf8(&vecmsg).unwrap();
    if (msg.chars().count() > 0){
        println!("Recived {:?}",msg);
    }
        let mut json_msg=json::parse(msg);
        json_msg
}

fn send_message(stream: &mut BufWriter<TcpStream>,message:String) -> (){
    let wbytes=stream.write(message.as_bytes());
    stream.write(b"\n");
    stream.flush();
    println!("Sent {:?} Bytes",wbytes.unwrap());
    // if wbytes==message.len(){
    //     true
    // } else {
    //     false
    // }
}


fn handle_client (stream: TcpStream){

    let stream2 = stream.try_clone().unwrap();
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream2);
    let mut watch_test=session::Instruction::WatchRangeInstruction{range:[33565418,33565418],exclude:Vec::new()};
    match watch_test.to_json(){
        Ok(str) => send_message(&mut writer,str),
        Err(e) => error!("{}",e),
    };
    // TODO: Do this in a new thread
    loop{
        let jsonData=read_message(&mut reader);
    }
}

fn main() -> std::io::Result<()>{
    

    let listener = TcpListener::bind("127.0.0.1:65398")?;

    //TODO: Remove this loop ?

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
