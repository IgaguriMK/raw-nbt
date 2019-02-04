use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use flate2::read::GzDecoder;

use raw_nbt::decode::Parser;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn load_idcounts() {
    match wrapped_load("./testdata/idcounts.dat", false) {
        Ok(_) => {}
        Err(e) => panic!(e.to_string()),
    }
}

#[test]
fn load_level() {
    match wrapped_load("./testdata/level.dat", true) {
        Ok(_) => {}
        Err(e) => panic!(e.to_string()),
    }
}

#[test]
fn load_map() {
    match wrapped_load("./testdata/map_9.dat", true) {
        Ok(_) => {}
        Err(e) => panic!(e.to_string()),
    }
}

#[test]
fn load_villages() {
    match wrapped_load("./testdata/villages.dat", true) {
        Ok(_) => {}
        Err(e) => panic!(e.to_string()),
    }
}

//// test util ////

fn wrapped_load(path: &str, gzipped: bool) -> Result<()> {
    let f = File::open(path)?;

    if gzipped {
        let r = GzDecoder::new(f);
        assert_parse(r)
    } else {
        let r = BufReader::new(f);
        assert_parse(r)
    }
}

fn assert_parse<R: Read>(r: R) -> Result<()> {
    let mut parser = Parser::new(r);

    match parser.parse() {
        Ok(nbt) => {
            eprintln!("{:?}", nbt);
        }
        Err(e) => return Err(Box::new(e)),
    }

    Ok(())
}
