use std::fmt::Debug;

#[derive(Debug)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl Version {
    pub fn from(major: u16, minor: u16) -> Self {
        Version { major, minor }
    }
}

pub trait BaseFBX: Debug {
    fn read(&mut self);
}

pub trait Node: Debug {}