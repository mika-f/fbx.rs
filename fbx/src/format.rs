pub(crate) use ascii::AsciiFBX;
pub use base::{Attribute, BaseFBXReader, BaseFBXWriter, Node, Object, Type};
pub(crate) use binary::BinaryFBX;

mod ascii;
mod base;
mod binary;