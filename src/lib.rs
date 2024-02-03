mod protocol;
mod client;
mod server;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc::channel;
    use std::thread;
    use crate::client::client::Client;
    use crate::protocol::{LibEvent, Message};
    use crate::server::server::Server;
    use super::*;

    #[test]
    fn it_works() {
        let (tx,rx) = channel();
        println!("hello");
        let handler = thread::spawn(move || {

            let mut  server = Server::new();
            server.begin_listening();
            loop {
                server.retrieve_messages();
                tx.send(server.pop_message()).expect("Unable to send on channel");
            }


        });




        let mut client = Client::new(String::from("127.0.0.1:42597"));

        client.send_message(Message::LibEvent(LibEvent::Ready));
        println!("message sent");

        let receiver = thread::spawn(move || {
            let value = rx.recv().expect("Unable to receive from channel");
            println!("Ho ricevuto con successo il messaggio {:?}",value);
        });


    }

}


