use std::{fmt::Debug, fs::File, io::BufReader};

use crate::format::base;

pub struct AsciiFBX {
    reader: BufReader<File>,
}

impl AsciiFBX {
    pub fn new(reader: BufReader<File>) -> Self {
        AsciiFBX { reader }
    }
}

impl base::BaseFBX for AsciiFBX {
    fn read(&mut self) {
        todo!()
    }
}

impl Debug for AsciiFBX {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsciiFBX").finish()
    }
}
