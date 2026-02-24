#!/usr/bin/env python3
"""
Generate simple test wasm files for the test suite.
This creates minimal valid WebAssembly binaries for testing different features.
"""

import struct
import os

OUTPUT_DIR = "tests/fixtures"

def write_wasm(name, bytes_data):
    """Write wasm bytes to file"""
    path = os.path.join(OUTPUT_DIR, f"{name}.wasm")
    with open(path, 'wb') as f:
        f.write(bytes(bytes_data))
    print(f"Created: {path}")

def minimal_module():
    """Just header - valid but empty"""
    return [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]

def type_section_only():
    """Module with just a type section"""
    wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]  # header
    # Type section
    wasm.append(0x01)  # section id
    wasm.append(0x05)  # section size
    wasm.append(0x01)  # 1 type
    wasm.append(0x60)  # func type
    wasm.append(0x00)  # 0 params
    wasm.append(0x01)  # 1 result
    wasm.append(0x7F)  # i32
    return wasm

def import_section():
    """Module with import section"""
    wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]  # header
    # Import section
    wasm.append(0x02)  # section id
    wasm.append(0x0C)  # section size
    wasm.append(0x01)  # 1 import
    wasm.append(0x03)  # module name len
    wasm.extend(b"env")
    wasm.append(0x04)  # field name len
    wasm.extend(b"test")
    wasm.append(0x00)  # import kind: func
    wasm.append(0x00)  # type index
    return wasm

def memory_section():
    """Module with memory section"""
    wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]  # header
    # Memory section
    wasm.append(0x05)  # section id
    wasm.append(0x03)  # section size
    wasm.append(0x01)  # 1 memory
    wasm.append(0x00)  # limits flag (no max)
    wasm.append(0x01)  # min = 1 page
    return wasm

def export_section():
    """Module with export section"""
    wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]  # header
    # Export section
    wasm.append(0x07)  # section id
    wasm.append(0x08)  # section size
    wasm.append(0x01)  # 1 export
    wasm.append(0x04)  # name len
    wasm.extend(b"main")
    wasm.append(0x00)  # export kind: func
    wasm.append(0x00)  # func index
    return wasm

def custom_section():
    """Module with custom section"""
    wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]  # header
    # Custom section
    wasm.append(0x00)  # section id
    wasm.append(0x06)  # section size
    wasm.append(0x03)  # name len
    wasm.extend(b"abc")
    wasm.append(0x01)
    wasm.append(0x02)  # custom data
    return wasm

def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    print("Generating test wasm files...")
    write_wasm("minimal", minimal_module())
    write_wasm("type_only", type_section_only())
    write_wasm("import_only", import_section())
    write_wasm("memory_only", memory_section())
    write_wasm("export_only", export_section())
    write_wasm("custom_only", custom_section())
    print("\nDone! Run: cargo test test_spec -- --ignored")

if __name__ == "__main__":
    main()
