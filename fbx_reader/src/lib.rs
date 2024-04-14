extern crate fbx;

use fbx::format::Object;

#[derive(Debug)]
pub struct FBXReader {}

impl FBXReader {
    pub fn from(fbx: Object) -> Self {
        FBXReader {}
    }
}

