use std::vec::IntoIter;
use crate::protocol::Message;

/// Wrapper structs over the messages that compose an ingame tick
/// The last message will always be either an EndOfTick" or a "Terminated"
#[derive(Debug)]
pub struct Tick{
    messages: Vec<Message>
}

impl Tick{
    pub fn new(vec: Vec<Message>)->Self{
        Self{
            messages:vec
        }
    }
}

impl IntoIterator for Tick{
    type Item = Message;
    type IntoIter = IntoIter<Message>;

    fn into_iter(self) -> Self::IntoIter {
      self.messages.into_iter()
    }
}
