use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::error::{ReadError, Result};
use crate::format::{BaseFBXReader, Object};

mod binary;
mod format;
mod error;


const FBX_MAGIC_BINARY_BYTES: [u8; 23] = [
    0x4b, 0x61, 0x79, 0x64, 0x61, 0x72, 0x61, 0x20, 0x46, 0x42, 0x58, 0x20, 0x42, 0x69, 0x6e, 0x61,
    0x72, 0x79, 0x20, 0x20, 0x00, 0x1a, 0x00,
];

fn has_fbx_magic_bytes(reader: &mut BufReader<File>) -> bool {
    let mut bytes: [u8; 23] = [0; 23];
    let _ = reader.read_exact(&mut bytes);

    bytes == FBX_MAGIC_BINARY_BYTES
}

pub fn read_fbx(path: &Path) -> Result<Object> {
    let file = std::fs::File::open(path).map_err(|_| {
        ReadError::FailedToOpenFile(path.to_owned().into_os_string().into_string().unwrap())
    })?;

    let mut reader = BufReader::new(file);
    let mut fbx: Box<dyn BaseFBXReader> = if has_fbx_magic_bytes(&mut reader) {
        Box::new(format::BinaryFBX::new(reader))
    } else {
        Box::new(format::AsciiFBX::new(reader))
    };

    fbx.read()
}
