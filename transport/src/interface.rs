use std::error::Error;
use crate::TransportValue;

pub trait InstructionParser {
    fn unpack_instruction(&self,content: &[u8]) -> Result<TransportValue, Box<dyn Error>>;
}

pub trait InterfaceRegistrar: Sync + Send {
    fn register_parser(&mut self, handler: Box<dyn InstructionParser + Send + Sync>);
}

#[derive(Copy, Clone)]
pub struct InstructionInterface {
    pub register: unsafe extern "C" fn(&mut dyn InterfaceRegistrar),
}