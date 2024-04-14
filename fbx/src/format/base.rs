use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::transmute;

#[derive(Debug, Eq, PartialEq, Ord, Clone, Copy)]
pub struct Version {
    major: u16,
    minor: u16,
}

impl PartialOrd<Self> for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.major > other.major {
            return Some(Ordering::Greater);
        }

        if self.major < other.major {
            return Some(Ordering::Less);
        }

        if self.minor > other.minor {
            return Some(Ordering::Greater);
        }

        if self.minor < other.minor {
            return Some(Ordering::Less);
        }

        return Some(Ordering::Equal);
    }
}

impl Version {
    pub fn from(major: u16, minor: u16) -> Self {
        Version { major, minor }
    }

    pub fn to_u8_le(self) -> [u8; 4] {
        let version: u32 = ((self.major as u32) * 1000) + ((self.minor as u32) * 100);
        unsafe { transmute::<u32, [u8; 4]>(version.to_le()) }
    }
}

pub trait BaseFBX: Debug {
    fn read(&mut self);
}

pub trait Attribute: Debug {}

#[derive(Debug)]
pub struct Node {
    name: String,
    attributes: Vec<Type>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, attributes: Vec<Type>, children: Vec<Node>) -> Self {
        Node { name, attributes, children }
    }
}

#[derive(Debug)]
pub enum Type {
    Bool(bool),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    VecBool(Vec<bool>),
    VecInt32(Vec<i32>),
    VecInt64(Vec<i64>),
    VecFloat32(Vec<f32>),
    VecFloat64(Vec<f64>),
    VecRaw(Vec<u8>),
    String(String),
}