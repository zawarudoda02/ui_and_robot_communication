use crate::errors::CommError;
use std::collections::VecDeque;
use std::io::{BufReader, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;


use crate::protocol::{ LibEvent, Message};

use crate::tick::Tick;


///
/// abstraction over a tcp server, used by the ui to listen to incoming messages from an ai
/// Only an ai may be connected at a time.
/// The server will read messages from the stream until an "EndOfTick", "Terminated" has been sent, or an error has been raised by the tcpstream/deserializer
///
///
pub struct Server {
    listener: TcpListener,
    stream: Option<BufReader<TcpStream>>,
    messages: VecDeque<Message>,
}
impl Server {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:42597").unwrap();
        println!("Connected on port 42597");
        Self {
            listener,
            stream: None,
            messages: VecDeque::new(),
        }
    }

    pub fn begin_listening(&mut self) -> Result<(),()>{
        let (socket, addr) = self.listener.accept().map_err(|_| { () })?;
        println!("This client connected: {:?}", addr);
        socket.set_read_timeout(Some(Duration::from_secs(10))).map_err(|_| { () })?;

        self.stream = Some(BufReader::new(socket));

        Ok(())
    }
    pub fn stop_listening(&mut self) {
        self.messages.clear();
        self.stream = None;
    }


    ///the server keeps listening until an "EndOfTick" or "Terminated" message is read, or returns an error in case of disconnection
    pub fn get_tick(&mut self)->Result<Tick,CommError>{
        loop{
            let a = self.retrieve_message()?;


            if let Message::LibEvent(LibEvent::Terminated)  | Message::LibEvent(LibEvent::EndOfTick)= a{
                let mut vec = vec![];

                while let Some(x) = self.messages.pop_front(){

                    vec.push(x);

                }
                vec.push(a);
                return Ok(Tick::new(vec));
            }
            self.messages.push_back(a);
        }
    }
    ///function to be called ONLY at the beginning, it gives a tick containing the World Info
    pub fn get_world_info(&mut self)->Result<Tick,CommError>{
        let a = self.retrieve_message()?;
        let b = self.retrieve_message()?;
        if let (Message::LibEvent(LibEvent::Ready), Message::WorldInfo {..}) = (a.clone(),b.clone()){
            return Ok(Tick::new(vec![a,b]));
        }
        return Err(CommError::FirstMessageIsNotWorldInfo);

    }
}
//private methods
impl Server{
    #[allow(dead_code)]
    fn pop_message(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }
    fn get_stream(&mut self) -> Result<&mut BufReader<TcpStream>, CommError> {
        match &mut self.stream {
            None => Err(CommError::NoClient),
            Some(x) => {
                let a = x.get_ref();
                if let Err(e) = Server::check_connection(&a){
                    return Err(e);
                }
                Ok(x)
            }
        }
    }
    fn check_connection(stream: &TcpStream)->Result<(),CommError>{
        let mut buff: [u8; 1] = [0; 1];

        match stream.peek(&mut buff) {
            Ok(_) => {
                Ok(())
            }
            Err(e) => {
                return match e.kind() {

                    ErrorKind::ConnectionRefused => {unreachable!() //si spera
                        /*Err(CommError::ClientDisconnected)*/},
                    ErrorKind::ConnectionReset => Err(CommError::ClientDisconnected),

                    ErrorKind::ConnectionAborted => Err(CommError::ClientDisconnected),
                    ErrorKind::NotConnected => Err(CommError::ClientDisconnected),

                    ErrorKind::TimedOut => Err(CommError::TimedOut),

                    _ => Ok(()),
                }
            }
        }
    }
    ///retrieves a message, returning an error if there are no more messages being broadcast
    fn retrieve_message(&mut self) -> Result<Message, CommError> {
        let stream = self.get_stream()?;

        match bincode::deserialize_from(stream) {
            Ok(e) => {
                return Ok(e);

            }
            Err(e) => {
                return Err(CommError::DeserializationError(e));
            }
        }


    }
}

