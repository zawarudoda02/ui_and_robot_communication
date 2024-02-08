use std::io;

#[derive(Debug)]
pub enum CommError{
    NoClient,
    ClientDisconnected,
    TimedOut,
    DeserializationError(bincode::Error),
    Timeout
}