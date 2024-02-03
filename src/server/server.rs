use std::collections::VecDeque;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use crate::protocol::Message;

pub struct Server{
    listener: TcpListener,
    stream: Option<TcpStream>,
    messages: VecDeque<Message>
}
impl Server{
    pub fn new()->Self{
        let listener = TcpListener::bind("127.0.0.1:42597").unwrap();
        println!("Connected on port 80");
        Self{
            listener,
            stream:None,
            messages:VecDeque::new()
        }

    }
    pub fn begin_listening(&mut self){

            let (socket,addr) = self.listener.accept().unwrap();
            println!("This client connected: {:?}",addr);

            self.stream = Some(socket);

    }
    pub fn stop_listening(&mut self){
        self.messages.clear();
        self.stream = None;
    }
    pub fn retrieve_messages(&mut self)->Result<(),()>{
        match &mut self.stream{
            None => {return Ok(());}
            Some(stream) => {

                match bincode::deserialize_from(stream){
                    Ok(e)=>{
                        self.messages.push_front(e);

                    }
                    Err(e)=>{
                        return Err(());
                    }
                }
            }

        }
        return Ok(())

    }
    pub fn pop_message(&mut self)-> Option<Message>{
        self.messages.pop_front()
    }
}