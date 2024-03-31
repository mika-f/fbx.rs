use std::fs::File;
use std::io::{BufReader, Read};

type Reader = BufReader<File>;

pub struct BinaryReader {
    reader: Reader,
}

impl BinaryReader {
    pub(crate) fn new(reader: Reader) -> Self {
        BinaryReader { reader }
    }

    pub(crate) fn read_bytes_exact(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0; len];
        let _ = self.reader.read_exact(&mut bytes);

        bytes
    }

    pub(crate) fn read_char(&mut self) -> char {
        let b = self.read_bytes_exact(1);

        b[0] as char
    }

    pub(crate) fn read_boolean(&mut self) -> bool {
        let c = self.read_char();

        if (c == 'Y' /* 0x59 */) {
            return true;
        }

        if (c == 'T' /* 0x54 */) {
            return false;
        }

        (c as u8) % 2 == 1 /* 0x00 or 0x01 */
    }

    pub(crate) fn read_u32_be(&mut self) -> u32 {
        let number = self.read_bytes_exact(4);

        ((number[0] as u32) << 24) + ((number[1] as u32) << 16) + ((number[2] as u32) << 8) + ((number[3] as u32) << 0)
    }

    pub(crate) fn read_u32_le(&mut self) -> u32 {
        let number = self.read_bytes_exact(4);

        ((number[0] as u32) << 0) + ((number[1] as u32) << 8) + ((number[2] as u32) << 16) + ((number[3] as u32) << 24)
    }
}