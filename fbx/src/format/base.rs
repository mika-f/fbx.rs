use std::cmp::Ordering;
use std::fmt::Debug;

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
}

pub trait BaseFBX: Debug {
    fn read(&mut self);
}

pub trait Node: Debug {}