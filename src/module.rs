use alloc::vec::Vec;

#[derive(Clone)]
pub enum Command {
    Set(usize, u8),
}

#[derive(Clone)]
pub struct Module {
    pub commands: Vec<Command>,
}

impl Module {}
