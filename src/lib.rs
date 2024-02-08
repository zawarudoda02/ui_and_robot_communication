mod protocol;
mod client;
mod server;
mod errors;
mod tick;


#[cfg(test)]
mod tests {
    use std::mem::{size_of, size_of_val};
    use std::sync::mpsc::channel;
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;
    use robotics_lib::world::tile::{Content, Tile, TileType};
    use crate::client::client::Client;
    use crate::errors::CommError;
    use crate::protocol::{LibEvent, Message};
    use crate::protocol::LibEvent::Terminated;
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
                let a = server.get_tick();
                match a{
                    Ok(x) => {tx.send(Ok(x));}
                    Err(e) => {tx.send(Err(e)); break;}
                }
            }


        });




        let mut client = Client::new(String::from("127.0.0.1:42597")).unwrap();

        client.send_message(Message::LibEvent(LibEvent::Ready));
        client.send_message(Message::LibEvent(LibEvent::EndOfTick));
        client.send_message(Message::LibEvent(LibEvent::ToolUsed));
        let tile_vec: Vec<(Tile,(usize,usize))> = vec![(Tile{
            tile_type: TileType::Mountain,
            content: Content::Building,
            elevation:69420
        },(10,10));10000];


        println!("\t\t\t\t Il messaggio ha una dimensione di: {:?}",bincode::serialized_size(&tile_vec));
        println!("{}",size_of::<(Tile,(usize,usize))>() * 10000);
        println!("{:?}",client.send_message(Message::LibEvent(LibEvent::DiscoveredTiles(tile_vec))));


        client.send_message(Message::LibEvent(Terminated));

        println!("message sent");

        client.send_message(Message::LibEvent(LibEvent::Terminated));
        let receiver = thread::spawn(move || {
            loop {

                let value = rx.recv().expect("Unable to receive from channel");
                println!("Ho ricevuto con successo il messaggio {:?}", value);
                if let Err(x) = value{
                    break;
                }
            }

        });

        sleep(Duration::from_millis(1000));
    }

}


