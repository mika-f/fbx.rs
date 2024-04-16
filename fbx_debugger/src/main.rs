extern crate fbx;

use std::path::Path;

use fbx::read_fbx;
use fbx_reader::FBXReader;

fn main() {
    let fbx = read_fbx(Path::new("./MANUKA.fbx")).unwrap();
    let reader = FBXReader::from(fbx);

    let extension = reader.get("FBXHeaderExtension").unwrap();
    let version = extension.get_node("FBXHeaderVersion").unwrap().get_value().unwrap().as_int32();
    assert_eq!(version, Some(1003));

    let creator = extension.get_node("Creator").unwrap().get_value().unwrap().as_str();
    assert_eq!(creator, Some("Blender (stable FBX IO) - 2.83.20 - 4.20.5".to_owned()));
}
