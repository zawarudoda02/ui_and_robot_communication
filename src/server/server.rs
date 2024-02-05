use std::collections::VecDeque;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use bincode::Error;

use crate::protocol::Message;

pub struct Server{
    listener: TcpListener,
    stream: Option<TcpStream>,
    messages: VecDeque<Message>
}
impl Server{
    pub fn new()->Self{
        let listener = TcpListener::bind("127.0.0.1:42597").unwrap();
        println!("Connected on port 42597");
        Self{
            listener,
            stream:None,
            messages:VecDeque::new()
        }


    }
    pub fn begin_listening(&mut self){

            let (socket,addr) = self.listener.accept().unwrap();
            println!("This client connected: {:?}",addr);
            socket.set_nonblocking(true).expect("Ah boh cazz ne so");

            self.stream = Some(socket);

    }
    pub fn stop_listening(&mut self){
        self.messages.clear();
        self.stream = None;
    }
    pub fn retrieve_messages(&mut self)->Result<(), Error>{

        match &mut self.stream{
            None => {return Ok(());}
            Some(stream) => {

                match bincode::deserialize_from(stream){
                    Ok(e)=>{
                        self.messages.push_front(e);

                    }
                    Err(e)=>{
                        return Err(e);
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