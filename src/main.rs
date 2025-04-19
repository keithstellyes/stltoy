use stltoy::readStlFile;
use std::path::Path;
use std::env;

fn main() {
    // pollster::block_on(run());
    let args: Vec<String> = env::args().collect();
    let filename: &String = &args[1];
    println!("FN: {}", filename);
    let obj = readStlFile(Path::new(filename)).unwrap();

    println!("{}", obj);
}
