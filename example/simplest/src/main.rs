use std::fs::File;
use std::io::prelude::*;
use webassembly::*;

fn main() -> std::io::Result<()> {
    // lets make a program with a single main function 
    // that returns 42, byte by byte

    // start with MAGIC_NUMBER
    let mut program = MAGIC_NUMBER.to_vec();
    // add the version
    program.extend_from_slice(VERSION_1)


    // write out the bytes
    let mut buffer = File::create("simplest.wasm")?;
    buffer.write(&program)?;
    Ok(())
}
