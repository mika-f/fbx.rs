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

    pub(crate) fn read_string(&mut self, length: usize) -> String {
        let bytes = self.read_bytes_exact(length);

        String::from_utf8(bytes).unwrap()
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

    pub(crate) fn read_u8(&mut self) -> u8 {
        self.read_bytes_exact(1)[0]
    }

    pub(crate) fn read_u16_be(&mut self) -> u16 {
        let number = self.read_bytes_exact(2);
        let bytes: [u8; 2] = number.try_into().unwrap();

        u16::from_be_bytes(bytes)
    }

    pub(crate) fn read_u16_le(&mut self) -> u16 {
        let number = self.read_bytes_exact(2);
        let bytes: [u8; 2] = number.try_into().unwrap();

        u16::from_le_bytes(bytes)
    }

    pub(crate) fn read_u32_be(&mut self) -> u32 {
        let number = self.read_bytes_exact(4);
        let bytes: [u8; 4] = number.try_into().unwrap();

        u32::from_be_bytes(bytes)
    }

    pub(crate) fn read_u32_le(&mut self) -> u32 {
        let number = self.read_bytes_exact(4);
        let bytes: [u8; 4] = number.try_into().unwrap();

        u32::from_le_bytes(bytes)
    }

    pub(crate) fn read_u64_be(&mut self) -> u64 {
        let number = self.read_bytes_exact(8);
        let bytes: [u8; 8] = number.try_into().unwrap();

        u64::from_be_bytes(bytes)
    }

    pub(crate) fn read_u64_le(&mut self) -> u64 {
        let number = self.read_bytes_exact(8);
        let bytes: [u8; 8] = number.try_into().unwrap();

        u64::from_le_bytes(bytes)
    }

    pub(crate) fn read_f32_be(&mut self) -> f32 {
        let number = self.read_bytes_exact(4);
        let bytes: [u8; 4] = number.try_into().unwrap();

        f32::from_be_bytes(bytes)
    }

    pub(crate) fn read_f32_le(&mut self) -> f32 {
        let number = self.read_bytes_exact(4);
        let bytes: [u8; 4] = number.try_into().unwrap();

        f32::from_le_bytes(bytes)
    }

    pub(crate) fn read_f64_be(&mut self) -> f64 {
        let number = self.read_bytes_exact(8);
        let bytes: [u8; 8] = number.try_into().unwrap();

        f64::from_be_bytes(bytes)
    }

    pub(crate) fn read_f64_le(&mut self) -> f64 {
        let number = self.read_bytes_exact(8);
        let bytes: [u8; 8] = number.try_into().unwrap();

        f64::from_le_bytes(bytes)
    }
}