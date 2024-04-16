use std::cmp::Ordering;
use std::fmt::Debug;
use std::mem::transmute;

use crate::error::Result;

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

pub trait BaseFBXReader: Debug {
    fn read(&mut self) -> Result<Object>;
}

pub trait BaseFBXWriter: Debug {}

pub trait Attribute: Debug {}

#[derive(Debug)]
pub struct Object {
    version: Version,
    children: Vec<Node>,
    footer: Option<Vec<u8>>,
}

impl Object {
    pub fn new(version: Version, children: Vec<Node>, footer: Option<Vec<u8>>) -> Self {
        Object { version, children, footer }
    }

    pub fn version(self) -> Version {
        self.version
    }

    pub fn children(&self) -> Vec<Node> {
        self.children.clone()
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    attributes: Vec<Type>,
    children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, attributes: Vec<Type>, children: Vec<Node>) -> Self {
        Node { name, attributes, children }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn attributes(&self) -> Vec<Type> {
        return self.attributes.clone();
    }

    pub fn children(&self) -> Vec<Node> {
        return self.children.clone();
    }
}


#[derive(Debug, Clone)]
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

impl Type {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Type::Bool(b) => Some(b.to_owned()),
            _ => None
        }
    }

    pub fn as_int16(&self) -> Option<i16> {
        match self {
            Type::Int16(i) => Some(i.to_owned()),
            _ => None
        }
    }

    pub fn as_int32(&self) -> Option<i32> {
        match self {
            Type::Int32(i) => Some(i.to_owned()),
            _ => None
        }
    }

    pub fn as_int64(&self) -> Option<i64> {
        match self {
            Type::Int64(i) => Some(i.to_owned()),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            Type::Float32(f) => Some(f.to_owned()),
            _ => None
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            Type::Float64(f) => Some(f.to_owned()),
            _ => None
        }
    }

    pub fn as_int32_array(&self) -> Option<Vec<i32>> {
        match self {
            Type::VecInt32(i) => Some(i.to_owned()),
            _ => None
        }
    }

    pub fn as_int64_array(&self) -> Option<Vec<i64>> {
        match self {
            Type::VecInt64(i) => Some(i.to_owned()),
            _ => None
        }
    }

    pub fn as_float_array(&self) -> Option<Vec<f32>> {
        match self {
            Type::VecFloat32(f) => Some(f.to_owned()),
            _ => None
        }
    }

    pub fn as_double_array(&self) -> Option<Vec<f64>> {
        match self {
            Type::VecFloat64(f) => Some(f.to_owned()),
            _ => None
        }
    }

    pub fn as_binary(&self) -> Option<Vec<u8>> {
        match self {
            Type::VecRaw(b) => Some(b.to_owned()),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<String> {
        match self {
            Type::String(s) => Some(s.to_owned()),
            _ => None
        }
    }
}