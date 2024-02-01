use std::net::{Ipv4Addr, TcpStream};
use crate::protocol::Message;

pub struct Client{
    stream: TcpStream
}
impl Client{
    pub fn new(ip: String)-> Self{
        let stream = TcpStream::connect(ip.to_string()).unwrap();
        println!("hello!");
        Self{
            stream
        }
    }
    pub fn send_message(&mut self,message: Message){
        bincode::serialize_into(&mut self.stream,&message).unwrap();
    }
}
