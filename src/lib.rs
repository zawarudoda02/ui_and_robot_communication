mod protocol;
mod client;
mod server;
mod errors;
mod tick;

pub use protocol::{LibEvent,Message,EventError};
pub use errors::CommError;
pub use tick::Tick;
pub use client::client::Client;
pub use server::server::Server;


#[cfg(test)]
mod tests {
    use std::mem::size_of;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use crate::client::client::Client;
    use crate::protocol::{LibEvent, Message};
    use crate::protocol::LibEvent::Terminated;
    use crate::server::server::Server;
    //use super::*;

    #[test]
    fn it_works() {
        let (tx,rx) = channel();
        println!("hello");
        let _handler = thread::spawn(move || {

            let mut  server = Server::new();
            server.begin_listening().expect("Client can't be accepted");
            loop {
                let a = server.get_tick();
                match a{
                    Ok(x) => {tx.send(Ok(x)).expect("Send between threads failed");}
                    Err(e) => {tx.send(Err(e)).expect("Send between threads failed"); break;}
                }
            }
        });




        let mut client = Client::new(String::from("127.0.0.1:42597")).unwrap();

        client.send_message(Message::LibEvent(LibEvent::Ready)).expect("Client send failed");
        client.send_message(Message::LibEvent(LibEvent::EndOfTick)).expect("Client send failed");
        client.send_message(Message::LibEvent(LibEvent::ToolUsed)).expect("Client send failed");
        let tile_vec: Vec<(Tile,(usize,usize))> = vec![(Tile{
            tile_type: TileType::Mountain,
            content: Content::Building,
            elevation:69420
        },(10,10));10000];


        println!("\t\t\t\t Il messaggio ha una dimensione di: {:?}",bincode::serialized_size(&tile_vec));
        println!("{}",size_of::<(Tile,(usize,usize))>() * 10000);
        println!("{:?}",client.send_message(Message::LibEvent(LibEvent::DiscoveredTiles(tile_vec))));


        client.send_message(Message::LibEvent(Terminated)).expect("Client send failed");

        println!("message sent");

        client.send_message(Message::LibEvent(Terminated)).expect("Client send failed");
        let _receiver = thread::spawn(move || {
            loop {

                let value = rx.recv().expect("Unable to receive from channel");
                println!("Ho ricevuto con successo il messaggio {:?}", value);
                if let Err(_) = value{
                    break;
                }
            }

        });

        sleep(Duration::from_millis(1000));
    }

}


