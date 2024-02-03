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
    use std::thread::sleep;
    use std::time::Duration;
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
                /*println!("Retrieve message ha restituito: {:?}",*/ server.retrieve_messages()/*)*/;
                if let Some(x) = server.pop_message() {
                    tx.send(x).expect("Unable to send on channel");
                }
            }


        });




        let mut client = Client::new(String::from("127.0.0.1:42597"));

        client.send_message(Message::LibEvent(LibEvent::Ready));
        client.send_message(Message::LibEvent(LibEvent::Terminated));
        client.send_message(Message::LibEvent(LibEvent::ToolUsed));

        println!("message sent");

        let receiver = thread::spawn(move || {
            loop {
                let value = rx.recv().expect("Unable to receive from channel");
                println!("Ho ricevuto con successo il messaggio {:?}", value);
            }
        });

        sleep(Duration::from_millis(1000));
    }

}


