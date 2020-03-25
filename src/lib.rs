#![no_std]
#[macro_use]
extern crate alloc;
use alloc::vec::Vec;
use micromath::F32Ext;

pub const MAGIC_NUMBER: &'static [i32] = &[0, 97, 115, 109];
pub const VERSION_1: &'static [i32] = &[1, 0, 0, 0];
pub const I32: u8 = 127;
pub const I64: u8 = 126;
pub const F32: u8 = 125;
pub const F64: u8 = 124;
pub const ANYFUNC: u8 = 112;
pub const FUNC: u8 = 96;
pub const EMPTY: u8 = 64;
pub const SECTION_CUSTOM: u8 = 0;
pub const SECTION_TYPE: u8 = 1;
pub const SECTION_IMPORT: u8 = 2;
pub const SECTION_FUNCTION: u8 = 3;
pub const SECTION_TABLE: u8 = 4;
pub const SECTION_MEMORY: u8 = 5;
pub const SECTION_GLOBAL: u8 = 6;
pub const SECTION_EXPORT: u8 = 7;
pub const SECTION_START: u8 = 8;
pub const SECTION_ELEMENT: u8 = 9;
pub const SECTION_CODE: u8 = 10;
pub const SECTION_DATA: u8 = 11;
pub const UNREACHABLE: u8 = 0;
pub const NOP: u8 = 1;
pub const BLOCK: u8 = 2;
pub const LOOP: u8 = 3;
pub const IF: u8 = 4;
pub const ELSE: u8 = 5;
pub const END: u8 = 11;
pub const BR: u8 = 12;
pub const BR_IF: u8 = 13;
pub const BR_TABLE: u8 = 14;
pub const RETURN: u8 = 15;
pub const CALL: u8 = 16;
pub const CALL_INDIRECT: u8 = 17;
pub const DROP: u8 = 26;
pub const SELECT: u8 = 27;
pub const LOCAL_GET: u8 = 32;
pub const LOCAL_SET: u8 = 33;
pub const LOCAL_TEE: u8 = 34;
pub const GLOBAL_GET: u8 = 35;
pub const GLOBAL_SET: u8 = 36;
pub const I32_LOAD: u8 = 40;
pub const I64_LOAD: u8 = 41;
pub const F32_LOAD: u8 = 42;
pub const F64_LOAD: u8 = 43;
pub const I32_LOAD8_S: u8 = 44;
pub const I32_LOAD8_U: u8 = 45;
pub const I32_LOAD16_S: u8 = 46;
pub const I32_LOAD16_U: u8 = 47;
pub const I64_LOAD8_S: u8 = 48;
pub const I64_LOAD8_U: u8 = 49;
pub const I64_LOAD16_S: u8 = 50;
pub const I64_LOAD16_U: u8 = 51;
pub const I64_LOAD32_S: u8 = 52;
pub const I64_LOAD32_U: u8 = 53;
pub const I32_STORE: u8 = 54;
pub const I64_STORE: u8 = 55;
pub const F32_STORE: u8 = 56;
pub const F64_STORE: u8 = 57;
pub const I32_STORE8: u8 = 58;
pub const I32_STORE16: u8 = 59;
pub const I64_STORE8: u8 = 60;
pub const I64_STORE16: u8 = 61;
pub const I64_STORE32: u8 = 62;
pub const CURRENT_MEMORY: u8 = 63;
pub const GROW_MEMORY: u8 = 64;
pub const I32_CONST: u8 = 65;
pub const I64_CONST: u8 = 66;
pub const F32_CONST: u8 = 67;
pub const F64_CONST: u8 = 68;
pub const I32_EQZ: u8 = 69;
pub const I32_EQ: u8 = 70;
pub const I32_NE: u8 = 71;
pub const I32_LT_S: u8 = 72;
pub const I32_LT_U: u8 = 73;
pub const I32_GT_S: u8 = 74;
pub const I32_GT_U: u8 = 75;
pub const I32_LE_S: u8 = 76;
pub const I32_LE_U: u8 = 77;
pub const I32_GE_S: u8 = 78;
pub const I32_GE_U: u8 = 79;
pub const I64_EQZ: u8 = 80;
pub const I64_EQ: u8 = 81;
pub const I64_NE: u8 = 82;
pub const I64_LT_S: u8 = 83;
pub const I64_LT_U: u8 = 84;
pub const I64_GT_S: u8 = 85;
pub const I64_GT_U: u8 = 86;
pub const I64_LE_S: u8 = 87;
pub const I64_LE_U: u8 = 88;
pub const I64_GE_S: u8 = 89;
pub const I64_GE_U: u8 = 90;
pub const F32_EQ: u8 = 91;
pub const F32_NE: u8 = 92;
pub const F32_LT: u8 = 93;
pub const F32_GT: u8 = 94;
pub const F32_LE: u8 = 95;
pub const F32_GE: u8 = 96;
pub const F64_EQ: u8 = 97;
pub const F64_NE: u8 = 98;
pub const F64_LT: u8 = 99;
pub const F64_GT: u8 = 100;
pub const F64_LE: u8 = 101;
pub const F64_GE: u8 = 102;
pub const I32_CLZ: u8 = 103;
pub const I32_CTZ: u8 = 104;
pub const I32_POPCNT: u8 = 105;
pub const I32_ADD: u8 = 106;
pub const I32_SUB: u8 = 107;
pub const I32_MUL: u8 = 108;
pub const I32_DIV_S: u8 = 109;
pub const I32_DIV_U: u8 = 110;
pub const I32_REM_S: u8 = 111;
pub const I32_REM_U: u8 = 112;
pub const I32_AND: u8 = 113;
pub const I32_OR: u8 = 114;
pub const I32_XOR: u8 = 115;
pub const I32_SHL: u8 = 116;
pub const I32_SHR_S: u8 = 117;
pub const I32_SHR_U: u8 = 118;
pub const I32_ROTL: u8 = 119;
pub const I32_ROTR: u8 = 120;
pub const I64_CLZ: u8 = 121;
pub const I64_CTZ: u8 = 122;
pub const I64_POPCNT: u8 = 123;
pub const I64_ADD: u8 = 124;
pub const I64_SUB: u8 = 125;
pub const I64_MUL: u8 = 126;
pub const I64_DIV_S: u8 = 127;
pub const I64_DIV_U: u8 = 128;
pub const I64_REM_S: u8 = 129;
pub const I64_REM_U: u8 = 130;
pub const I64_AND: u8 = 131;
pub const I64_OR: u8 = 132;
pub const I64_XOR: u8 = 133;
pub const I64_SHL: u8 = 134;
pub const I64_SHR_S: u8 = 135;
pub const I64_SHR_U: u8 = 136;
pub const I64_ROTL: u8 = 137;
pub const I64_ROTR: u8 = 138;
pub const F32_ABS: u8 = 139;
pub const F32_NEG: u8 = 140;
pub const F32_CEIL: u8 = 141;
pub const F32_FLOOR: u8 = 142;
pub const F32_TRUNC: u8 = 143;
pub const F32_NEAREST: u8 = 144;
pub const F32_SQRT: u8 = 145;
pub const F32_ADD: u8 = 146;
pub const F32_SUB: u8 = 147;
pub const F32_MUL: u8 = 148;
pub const F32_DIV: u8 = 149;
pub const F32_MIN: u8 = 150;
pub const F32_MAX: u8 = 151;
pub const F32_COPYSIGN: u8 = 152;
pub const F64_ABS: u8 = 153;
pub const F64_NEG: u8 = 154;
pub const F64_CEIL: u8 = 155;
pub const F64_FLOOR: u8 = 156;
pub const F64_TRUNC: u8 = 157;
pub const F64_NEAREST: u8 = 158;
pub const F64_SQRT: u8 = 159;
pub const F64_ADD: u8 = 160;
pub const F64_SUB: u8 = 161;
pub const F64_MUL: u8 = 162;
pub const F64_DIV: u8 = 163;
pub const F64_MIN: u8 = 164;
pub const F64_MAX: u8 = 165;
pub const F64_COPYSIGN: u8 = 166;
pub const I32_WRAP_F64: u8 = 167;
pub const I32_TRUNC_S_F32: u8 = 168;
pub const I32_TRUNC_U_F32: u8 = 169;
pub const I32_TRUNC_S_F64: u8 = 170;
pub const I32_TRUNC_U_F64: u8 = 171;
pub const I64_EXTEND_S_I32: u8 = 172;
pub const I64_EXTEND_U_I32: u8 = 173;
pub const I64_TRUNC_S_F32: u8 = 174;
pub const I64_TRUNC_U_F32: u8 = 175;
pub const I64_TRUNC_S_F64: u8 = 176;
pub const I64_TRUNC_U_F64: u8 = 177;
pub const F32_CONVERT_S_I32: u8 = 178;
pub const F32_CONVERT_U_I32: u8 = 179;
pub const F32_CONVERT_S_I64: u8 = 180;
pub const F32_CONVERT_U_I64: u8 = 181;
pub const F32_DEMOTE_F64: u8 = 182;
pub const F64_CONVERT_S_I32: u8 = 183;
pub const F64_CONVERT_U_I32: u8 = 184;
pub const F64_CONVERT_S_I64: u8 = 185;
pub const F64_CONVERT_U_I64: u8 = 186;
pub const F64_PROMOTE_F32: u8 = 187;
pub const I32_REINTERPRET_F32: u8 = 188;
pub const I64_REINTERPRET_F64: u8 = 189;
pub const F32_REINTERPRET_I32: u8 = 190;
pub const F64_REINTERPRET_I64: u8 = 191;
pub const DESC_FUNCTION: u8 = 0;
pub const DESC_TABLE: u8 = 1;
pub const DESC_MEMORY: u8 = 2;
pub const DESC_GLOBAL: u8 = 3;
pub const LIMIT_MIN: u8 = 0;
pub const LIMIT_MIN_MAX: u8 = 1;
pub const IMMUTABLE: u8 = 0;
pub const MUTABLE: u8 = 1;

pub trait TypeWasmExt {
    fn as_wasm_bytes(self) -> Vec<u8>;
}

impl TypeWasmExt for u32 {
    fn as_wasm_bytes(self) -> Vec<u8> {
        let mut value = self;
        let mut bytes: Vec<u8> = vec![];
        while {
            let mut byte = (value & 0x7F) as u8;
            value = value >> 0x07;
            if value != 0 {
                byte = byte | 0x80;
            }
            bytes.push(byte);
            value != 0
        } {}
        bytes
    }
}

impl TypeWasmExt for i32 {
    fn as_wasm_bytes(self) -> Vec<u8> {
        let mut value = self;
        let mut bytes: Vec<u8> = vec![];
        let size = (value as f32).abs().log2().ceil() as i32;
        let negative = value < 0;
        let mut more = true;
        while more {
            let mut byte = (value & 127) as u8;
            value = value >> 7;

            if negative {
                value = value | (-(1 << (size - 7)));
            }

            if (value == 0 && ((byte & 0x40) == 0)) || (value == -1 && ((byte & 0x40) == 0x40)) {
                more = false;
            } else {
                byte = byte | 128;
            }

            bytes.push(byte);
        }
        bytes
    }
}

impl TypeWasmExt for f64 {
    fn as_wasm_bytes(self) -> Vec<u8> {
        let raw_bytes: [u8; 8] = unsafe { core::mem::transmute(self) };
        let bytes: Vec<u8> = raw_bytes.to_vec();
        bytes
    }
}

impl TypeWasmExt for f32 {
    fn as_wasm_bytes(self) -> Vec<u8> {
        let raw_bytes: [u8; 4] = unsafe { core::mem::transmute(self) };
        let bytes: Vec<u8> = raw_bytes.to_vec();
        bytes
    }
}
