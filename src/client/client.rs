use std::net::{Ipv4Addr, Shutdown, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use crate::protocol::Message;
///
/// abstraction over a tcp client, used by the AI to communicate messages to the UI
///
///
/// NOTICE: after every game tick has ended, you must send an "EndOfTick" message, and at the end of the  robot's lifecycle send a "terminated" message, otherwise the program might not work correctly
///
///
///
pub struct Client{
    stream: TcpStream
}
impl Client{
    pub fn new(ip: String)->Result<Self,std::io::Error>{
        sleep(Duration::from_millis(1000));
        let stream = TcpStream::connect(ip)?;
        println!("hello!");
        Ok(Self{
            stream
        })
    }
    pub fn send_message(&mut self,message: Message)->Result<(),bincode::Error>{

        bincode::serialize_into(&mut self.stream,&message)

    }
    pub fn disconnect(&mut self){
        self.stream.shutdown(Shutdown::Both);
    }

}
