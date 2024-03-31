use std::path::Path;

use fbx::read_fbx;

extern crate fbx;

fn main() {
    let fbx = read_fbx(Path::new("./MANUKA.fbx")).unwrap();
    dbg!(&fbx);
}
