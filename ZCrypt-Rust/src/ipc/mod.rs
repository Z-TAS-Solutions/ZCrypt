use std::io::Result;

pub trait IPCHandler {
    fn handle(&mut self, request: Vec<u8>) -> Vec<u8>;
}
