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

}

impl base::BaseFBX for BinaryFBX {
    fn read(&mut self) {
    }
}

impl Debug for BinaryFBX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BinaryFBX")
            .finish()
    }
}
