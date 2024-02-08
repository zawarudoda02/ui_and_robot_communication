use crate::errors::CommError;
use bincode::Error;
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

    pub fn begin_listening(&mut self) {
        let (socket, addr) = self.listener.accept().unwrap();
        println!("This client connected: {:?}", addr);
        socket.set_read_timeout(Some(Duration::from_secs(10)));

        self.stream = Some(BufReader::new(socket));
    }
    pub fn stop_listening(&mut self) {
        self.messages.clear();
        self.stream = None;
    }


    ///the server keeps listening until an "EndOfTick" or "Terminated" message is read, or returns an error in case of disconnection
    pub fn get_tick(&mut self)->Result<Tick,CommError>{
        loop{
            let a = self.retrieve_message()?;
            if let ControlFlow::TickEnded  | ControlFlow::Terminated = a{
                let mut vec = vec![];
                while let Some(x) = self.messages.pop_front(){

                    vec.push(x);
                    if let Message::LibEvent(LibEvent::Terminated)| Message::LibEvent(LibEvent::EndOfTick) = vec.last().unwrap(){
                        return Ok(Tick::new(vec))
                    }
                }
            }

        }
    }
}
//private methods
impl Server{
    fn pop_message(&mut self) -> Option<Message> {
        self.messages.pop_front()
    }
    fn get_stream(&mut self) -> Result<&mut BufReader<TcpStream>, CommError> {
        match &mut self.stream {
            None => return Err(CommError::NoClient),
            Some(x) => {
                let a = x.get_ref();
                if let Err(e) = Server::check_connection(&a){
                    return Err(e);
                }
                return Ok((x));
            }
        }
    }
    fn check_connection(stream: &TcpStream)->Result<(),CommError>{
        let mut buff: [u8; 1] = [0; 1];

        match stream.peek(&mut buff) {
            Ok(_) => {
                return Ok(());
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
    fn retrieve_message(&mut self) -> Result<ControlFlow, CommError> {
        let stream = self.get_stream()?;
        let mut control_flow = ControlFlow::SameTick;
        match bincode::deserialize_from(stream) {
            Ok(e) => {
                match e{
                    Message::LibEvent(LibEvent::EndOfTick)=>{
                        control_flow=  ControlFlow::TickEnded
                    }
                    Message::LibEvent(LibEvent::Terminated)=>{
                        control_flow= ControlFlow::Terminated
                    }
                    _ => {}
                }
                self.messages.push_back(e);
            }
            Err(e) => {
                return Err(CommError::DeserializationError(e));
            }
        }

        return Ok(control_flow);
    }
}
///Enum used to check whether the message read is part of the same tick, the tick has ended or a robot's lifecycle has been terminated
#[derive(Debug,Clone)]
enum ControlFlow{
    Terminated,
    TickEnded,
    SameTick
}