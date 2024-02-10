
#[derive(Debug)]
pub enum CommError{
    NoClient,
    ClientDisconnected,
    TimedOut,
    DeserializationError(bincode::Error),
    Timeout
}