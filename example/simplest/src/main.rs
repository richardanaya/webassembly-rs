use std::fs::File;
use std::io::prelude::*;
use webassembly::*;

fn main() -> std::io::Result<()> {
    // lets make a program with a single main function 
    // that returns 42, byte by byte

    // start with MAGIC_NUMBER
    let mut program = MAGIC_NUMBER.to_vec();
    // add the version
    program.extend_from_slice(VERSION_1);

    // MAKE THE TYPE SECTION
    {
        program.push(SECTION_TYPE); // start the section with it's type id
        let mut sec = vec![];
        sec.extend_from_slice(&1u32.to_wasm_bytes()); // number of function signatures (only 1 for main)
        sec.push(FUNC); // start of first function signature
        sec.extend_from_slice(&0u32.to_wasm_bytes()); // first function signature has zero inputs
        sec.extend_from_slice(&1u32.to_wasm_bytes()); // first function signature has one output
        sec.push(I32); // first output is of type i32
        program.extend_from_slice(&sec.len().to_wasm_bytes()); // push the byte length of the section
        program.extend_from_slice(&sec); // push the data of the section
    }
        
    // MAKE THE FUNCTION SECTION
    {
        program.push(SECTION_FUNCTION);
        let mut sec = vec![];
        sec.extend_from_slice(&1u32.to_wasm_bytes()); // number of functions (only 1 for main)
        sec.extend_from_slice(&0u32.to_wasm_bytes()); // the first function should use the first signature we defined above
        program.extend_from_slice(&sec.len().to_wasm_bytes()); // push the byte length of the section
        program.extend_from_slice(&sec); // push the data of the section
    }

    // MAKE THE MEMORY SECTION
    {
        program.push(SECTION_MEMORY);
        let mut sec = vec![];
        sec.extend_from_slice(&1u32.to_wasm_bytes()); // number of memory blocks (only 1 for this program)
        sec.push(LIMIT_MIN_MAX); // first memory block defines min and max pages
        sec.extend_from_slice(&2u32.to_wasm_bytes()); // first memory block min pages
        sec.extend_from_slice(&10u32.to_wasm_bytes()); // first memory block max pages
        program.extend_from_slice(&sec.len().to_wasm_bytes()); // push the byte length of the section
        program.extend_from_slice(&sec); // push the data of the section
    }

    // MAKE THE EXPORT SECTION
    {
        program.push(SECTION_EXPORT);
        let mut sec = vec![];
        sec.extend_from_slice(&2u32.to_wasm_bytes()); // number of exports ( we have 1 function and 1 memory )

        let name = b"main";
        sec.extend_from_slice(&name.len().to_wasm_bytes()); // first export name length
        sec.extend_from_slice(name); // first export name data
        sec.push(DESC_FUNCTION); // first export is memory
        sec.extend_from_slice(&0u32.to_wasm_bytes()); // first export is of the first function

        let name = b"memory";
        sec.extend_from_slice(&name.len().to_wasm_bytes()); // second export name length
        sec.extend_from_slice(name); // second export name data
        sec.push(DESC_MEMORY); // second export is memory
        sec.extend_from_slice(&0u32.to_wasm_bytes()); // second export is of the first memory block

        program.extend_from_slice(&sec.len().to_wasm_bytes()); // push the byte length of the section
        program.extend_from_slice(&sec); // push the data of the section
    }

    // MAKE THE CODE SECTION
    {
        program.push(SECTION_CODE);
        let mut sec = vec![];

        // code block for "main"
        let mut code = vec![];
        // we have zero local variables, if we did we'd specify their count and types here
        code.extend_from_slice(&0u32.to_wasm_bytes()); 
        code.extend_from_slice(&[
            I32_CONST, 42,  // return 42
            END             // end function
        ]);

        sec.extend_from_slice(&1.to_wasm_bytes()); // number of code blocks (only 1 for main)
        sec.extend_from_slice(&code.len().to_wasm_bytes()); // length of first code block for "main"
        sec.extend_from_slice(&code); // data of first code block for "main"
        program.extend_from_slice(&sec.len().to_wasm_bytes()); // push the byte length of the section
        program.extend_from_slice(&sec); // push the data of the section
    }

    // write out the bytes
    let mut buffer = File::create("simplest.wasm")?;
    buffer.write(&program)?;
    Ok(())
}
