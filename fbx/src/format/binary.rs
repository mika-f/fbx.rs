use std::{fmt::Debug, fs::File, io::BufReader};

use crate::binary::BinaryReader;
use crate::format::base;
use crate::format::base::Version;

pub struct BinaryFBX {
    reader: BinaryReader,
    version: Option<Version>,
}

impl BinaryFBX {
    pub fn new(reader: BufReader<File>) -> Self {
        BinaryFBX { reader: BinaryReader::new(reader), version: None }
    }

    fn read_version(&mut self) {
        let num = self.reader.read_u32_le();

        let major = num / 1000;
        let minor = (num - major * 1000) / 100;

        self.version = Some(Version::from(major as u16, minor as u16))
    }

}

impl base::BaseFBX for BinaryFBX {
    fn read(&mut self) {
        self.read_version(); // 4 bytes
    }
}

impl Debug for BinaryFBX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryFBX")
            .field("version", &self.version)
            .finish()
    }
}
