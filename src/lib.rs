// 2 scenarios for meta data
// 1: ASCII STL file (only metadata is the name)
// 2: Binary STL file (meta data is the arbitrary 80 octet header (plus technically number of tris)

use std::path::Path;
use std::io;
use std::fmt;
use std::fs;
use std::error::Error;

pub struct StlMetaData {
}

pub struct StlVert {
    points: [f32; 3]
}

pub struct StlTriangle {
    normal: StlVert,
    points: [StlVert; 3]
}

pub struct StlObject {
    metadata: StlMetaData,
    triangles: Box<[StlTriangle]>
}

impl fmt::Display for StlObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "# tris: {}", (*self.triangles).len())
    }
}

fn parseAsciiStl(data: &str) -> io::Result<StlObject> {
    todo!()
}

fn parseBinStl(data: Vec<u8>) -> io::Result<StlObject> {
    todo!()
}

// have to use "dyn" because it's a trait object
fn parseStlBytes(data: Vec<u8>) -> Result<StlObject, Box<dyn Error>> {
    let md: StlMetaData = StlMetaData{};
    let tris: Box<[StlTriangle]> = Box::from([]);

    if data.len() < "solid\nendsolid".len() {
        // TODO: what does .into() do here, exactly?
        return Err("Invalid STL".into());
    }

    todo!("parseStlBytes")
}

pub fn readStlFile(p: &Path) -> Result<StlObject, Box<dyn Error>> {
    let bytes = fs::read(p)?;

    parseStlBytes(bytes)
}
