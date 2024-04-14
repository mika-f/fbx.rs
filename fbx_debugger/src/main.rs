extern crate fbx;

use std::path::Path;

use fbx::read_fbx;
use fbx_reader::FBXReader;

fn main() {
    let fbx = read_fbx(Path::new("./MANUKA.fbx")).unwrap();
    let reader = FBXReader::from(fbx);
    dbg!(&reader);
}
