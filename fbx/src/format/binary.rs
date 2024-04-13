use std::{fmt::Debug, fs::File, io::BufReader};
use std::io::Cursor;
use std::io::prelude::*;

use flate2::read::GzDecoder;

use crate::binary::BinaryReader;
use crate::format::base::{BaseFBX, Node, Version};
use crate::format::Type;
use crate::format::Type::{Bool, Float32, Float64, Int16, Int32, Int64, String, VecBool, VecFloat32, VecFloat64, VecInt32, VecInt64, VecRaw};

fn times(len: usize) -> impl Iterator {
    std::iter::repeat(()).take(len)
}

pub struct BinaryFBX {
    reader: BinaryReader,
    version: Option<Version>,
}

impl BinaryFBX {
    pub fn new(reader: BufReader<File>) -> Self {
        BinaryFBX { reader: BinaryReader::new(Box::new(reader), 23), version: None }
    }

    fn is_new_format(&self) -> bool {
        let boundary_version = Version::from(7, 5);
        let current_version = match self.version {
            Some(v) => v,
            None => panic!(), // invalid operation
        };

        current_version >= boundary_version
    }

    fn read_version(&mut self) {
        let num = self.reader.read_u32_le();

        let major = num / 1000;
        let minor = (num - major * 1000) / 100;

        self.version = Some(Version::from(major as u16, minor as u16))
    }

    fn read_vector<T>(&mut self, reader: impl Fn(&mut BinaryReader) -> T) -> Vec<T> {
        let length = self.reader.read_u32_le();
        let encoding = self.reader.read_u32_le();
        let bytes_length = self.reader.read_u32_le();
        let mut vec: Vec<T> = vec![];

        if encoding == 0 {
            // raw
            for _ in times(length as usize) {
                vec.push(reader(&mut self.reader))
            }
        } else if encoding == 1 {
            // zlib compressed
            let v = self.reader.read_bytes_exact(bytes_length as usize);
            let mut decoder = GzDecoder::new(Cursor::new(v));
            let mut br = BinaryReader::new(Box::new(decoder), 0);

            for _ in times(length as usize) {
                vec.push(reader(&mut br))
            }
        }

        vec
    }

    fn read_nodes(&mut self) -> Vec<Node> {
        let mut vec: Vec<Node> = vec![];

        loop {
            let offset: u64 = if self.is_new_format() { self.reader.read_u64_le() } else { self.reader.read_u32_le() as u64 };
            let attribute_length: u64 = if self.is_new_format() { self.reader.read_u64_le() } else { self.reader.read_u32_le() as u64 };
            let total_bytes: u64 = if self.is_new_format() { self.reader.read_u64_le() } else { self.reader.read_u32_le() as u64 };
            let name_length = self.reader.read_u8();

            if offset == 0 && attribute_length == 0 && total_bytes == 0 && name_length == 0 {
                break; // its node ending marker
            }

            let name = self.reader.read_string(name_length as usize);
            let mut attributes: Vec<Type> = vec![];

            for _ in times(attribute_length as usize) {
                let t = match self.reader.read_char() {
                    'C' => Bool(self.reader.read_boolean()),
                    'Y' => Int16(self.reader.read_i16_le()),
                    'I' => Int32(self.reader.read_i32_le()),
                    'L' => Int64(self.reader.read_i64_le()),
                    'F' => Float32(self.reader.read_f32_le()),
                    'D' => Float64(self.reader.read_f64_le()),
                    'b' => VecBool(self.read_vector(|w| w.read_boolean())),
                    'i' => VecInt32(self.read_vector(|w| w.read_i32_le())),
                    'l' => VecInt64(self.read_vector(|w| w.read_i64_le())),
                    'f' => VecFloat32(self.read_vector(|w| w.read_f32_le())),
                    'd' => VecFloat64(self.read_vector(|w| w.read_f64_le())),
                    'R' => {
                        let bytes_read = self.reader.read_u32_le();
                        VecRaw(self.reader.read_bytes_exact(bytes_read as usize))
                    }
                    'S' => {
                        let bytes_read = self.reader.read_u32_le();
                        String(self.reader.read_string(bytes_read as usize))
                    }
                    _ => panic!(), // invalid operation
                };

                attributes.push(t);
            }

            let mut children: Vec<Node> = vec![];

            loop {
                let cursor = self.reader.current_cursor();
                if (offset as usize) <= cursor {
                    break;
                }

                dbg!(cursor);
                children.extend(self.read_nodes());
            }

            vec.push(Node::new(
                name,
                attributes,
            ))
        }

        vec
    }

}

impl BaseFBX for BinaryFBX {
    fn read(&mut self) {
        self.read_version(); // 4 bytes
        self.read_nodes();   // unknown bytes
    }
}

impl Debug for BinaryFBX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryFBX")
            .field("version", &self.version)
            .finish()
    }
}
