// 2 scenarios for meta data
// 1: ASCII STL file (only metadata is the name)
// 2: Binary STL file (meta data is the arbitrary 80 octet header (plus technically number of tris)

use std::path::Path;
use std::io;
use std::fmt;
use std::fs;
use std::error::Error;
use std::str;

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

fn parseAsciiStl(data: &str) -> Result<StlObject, Box<dyn Error>> {
    todo!("ascii stl")
}

fn parseBinStl(data: Vec<u8>) -> Result<StlObject, Box<dyn Error>> {
    todo!("binary stl")
}

const fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

const MIN_ASCII_LEN: usize = "solid\nendsolid".len();
const MIN_BIN_LEN: usize = 80 /*header*/ + 4 /*triangle count*/;
const MIN_LEN: usize = min(MIN_ASCII_LEN, MIN_BIN_LEN);

// have to use "dyn" because it's a trait object
fn parseStlBytes(data: Vec<u8>) -> Result<StlObject, Box<dyn Error>> {
    let md: StlMetaData = StlMetaData{};
    let tris: Box<[StlTriangle]> = Box::from([]);

    if data.len() < MIN_LEN {
        // TODO: what does .into() do here, exactly?
        return Err("Invalid STL".into());
    }

    if &data[.."solid".len()] == "solid".as_bytes() {
        parseAsciiStl(str::from_utf8(&data)?)
    } else {
        parseBinStl(data)
    }
}

pub fn readStlFile(p: &Path) -> Result<StlObject, Box<dyn Error>> {
    let bytes = fs::read(p)?;

    parseStlBytes(bytes)
}
