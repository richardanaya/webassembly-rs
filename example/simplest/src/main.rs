use std::fs::File;
use std::io::prelude::*;
use webassembly::*;

fn main() -> std::io::Result<()> {
    // lets make a program with a single main function 
    // that returns 42, byte by byte
    let mut program: Vec<u8> = vec![];
    program.extend_from_slice(MAGIC_NUMBER);

    // write out the bytes
    let mut buffer = File::create("simplest.wasm")?;
    buffer.write(&program)?;
    Ok(())
}
