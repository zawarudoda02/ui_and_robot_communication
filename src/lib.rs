mod protocol;
mod client;
mod server;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::client::client::Client;
    use crate::protocol::{LibEvent, Message};
    use crate::server::server::Server;
    use super::*;

    #[test]
    fn it_works() {
        let mut  server = Server::new();
        server.begin_listening();
        let mut client = Client::new(String::from("127.0.0.1:80"));

        client.send_message(Message::LibEvent(LibEvent::Ready));

        server.retrieve_messages();
        println!("{:?}",server.pop_message());
    }
}


