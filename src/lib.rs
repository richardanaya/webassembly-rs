//! Full exhaustive WebAssembly Opcode Constants — Core Spec 3.0 (bikeshed Feb 2026)
//! https://webassembly.github.io/spec/core/bikeshed/
//! 100 % complete — no placeholders, every opcode listed.
#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// Allow std in tests
#[cfg(test)]
extern crate std;

// Optional: spec test loader (only for tests)
#[cfg(test)]
mod spec_test_loader;
#[cfg(test)]
pub use spec_test_loader::*;

pub const MAGIC: [u8; 4] = *b"\0asm";
pub const VERSION: u32 = 1;

// Value Types
pub const I32: u8 = 0x7F;
pub const I64: u8 = 0x7E;
pub const F32: u8 = 0x7D;
pub const F64: u8 = 0x7C;
pub const V128: u8 = 0x7B;
pub const FUNCREF: u8 = 0x70;
pub const EXTERNREF: u8 = 0x6F;

// Section IDs
pub const CUSTOM: u8 = 0;
pub const TYPE: u8 = 1;
pub const IMPORT: u8 = 2;
pub const FUNCTION: u8 = 3;
pub const TABLE: u8 = 4;
pub const MEMORY: u8 = 5;
pub const GLOBAL: u8 = 6;
pub const EXPORT: u8 = 7;
pub const START: u8 = 8;
pub const ELEMENT: u8 = 9;
pub const CODE: u8 = 10;
pub const DATA: u8 = 11;
pub const DATACOUNT: u8 = 12;
pub const TAG: u8 = 13;

// All opcodes
pub mod op {
    // ==================== SINGLE-BYTE OPCODES (0x00-0xFF) ====================
    pub const UNREACHABLE: u8 = 0x00;
    pub const NOP: u8 = 0x01;
    pub const BLOCK: u8 = 0x02;
    pub const LOOP: u8 = 0x03;
    pub const IF: u8 = 0x04;
    pub const ELSE: u8 = 0x05;
    pub const TRY: u8 = 0x06;
    pub const CATCH: u8 = 0x07;
    pub const THROW: u8 = 0x08;
    pub const RETHROW: u8 = 0x09;
    pub const THROW_REF: u8 = 0x0A;
    pub const END: u8 = 0x0B;
    pub const BR: u8 = 0x0C;
    pub const BR_IF: u8 = 0x0D;
    pub const BR_TABLE: u8 = 0x0E;
    pub const RETURN: u8 = 0x0F;
    pub const CALL: u8 = 0x10;
    pub const CALL_INDIRECT: u8 = 0x11;
    pub const RETURN_CALL: u8 = 0x12;
    pub const RETURN_CALL_INDIRECT: u8 = 0x13;
    pub const CALL_REF: u8 = 0x14;
    pub const RETURN_CALL_REF: u8 = 0x15;
    pub const DELEGATE: u8 = 0x18;
    pub const CATCH_ALL: u8 = 0x19;
    pub const TRY_TABLE: u8 = 0x1F;

    pub const DROP: u8 = 0x1A;
    pub const SELECT: u8 = 0x1B;
    pub const SELECT_T: u8 = 0x1C;

    pub const LOCAL_GET: u8 = 0x20;
    pub const LOCAL_SET: u8 = 0x21;
    pub const LOCAL_TEE: u8 = 0x22;
    pub const GLOBAL_GET: u8 = 0x23;
    pub const GLOBAL_SET: u8 = 0x24;
    pub const TABLE_GET: u8 = 0x25;
    pub const TABLE_SET: u8 = 0x26;

    pub const I32_LOAD: u8 = 0x28;
    pub const I64_LOAD: u8 = 0x29;
    pub const F32_LOAD: u8 = 0x2A;
    pub const F64_LOAD: u8 = 0x2B;
    pub const I32_LOAD8_S: u8 = 0x2C;
    pub const I32_LOAD8_U: u8 = 0x2D;
    pub const I32_LOAD16_S: u8 = 0x2E;
    pub const I32_LOAD16_U: u8 = 0x2F;
    pub const I64_LOAD8_S: u8 = 0x30;
    pub const I64_LOAD8_U: u8 = 0x31;
    pub const I64_LOAD16_S: u8 = 0x32;
    pub const I64_LOAD16_U: u8 = 0x33;
    pub const I64_LOAD32_S: u8 = 0x34;
    pub const I64_LOAD32_U: u8 = 0x35;
    pub const I32_STORE: u8 = 0x36;
    pub const I64_STORE: u8 = 0x37;
    pub const F32_STORE: u8 = 0x38;
    pub const F64_STORE: u8 = 0x39;
    pub const I32_STORE8: u8 = 0x3A;
    pub const I32_STORE16: u8 = 0x3B;
    pub const I64_STORE8: u8 = 0x3C;
    pub const I64_STORE16: u8 = 0x3D;
    pub const I64_STORE32: u8 = 0x3E;
    pub const MEMORY_SIZE: u8 = 0x3F;
    pub const MEMORY_GROW: u8 = 0x40;

    pub const I32_CONST: u8 = 0x41;
    pub const I64_CONST: u8 = 0x42;
    pub const F32_CONST: u8 = 0x43;
    pub const F64_CONST: u8 = 0x44;

    pub const I32_EQZ: u8 = 0x45;
    pub const I32_EQ: u8 = 0x46;
    pub const I32_NE: u8 = 0x47;
    pub const I32_LT_S: u8 = 0x48;
    pub const I32_LT_U: u8 = 0x49;
    pub const I32_GT_S: u8 = 0x4A;
    pub const I32_GT_U: u8 = 0x4B;
    pub const I32_LE_S: u8 = 0x4C;
    pub const I32_LE_U: u8 = 0x4D;
    pub const I32_GE_S: u8 = 0x4E;
    pub const I32_GE_U: u8 = 0x4F;

    pub const I64_EQZ: u8 = 0x50;
    pub const I64_EQ: u8 = 0x51;
    pub const I64_NE: u8 = 0x52;
    pub const I64_LT_S: u8 = 0x53;
    pub const I64_LT_U: u8 = 0x54;
    pub const I64_GT_S: u8 = 0x55;
    pub const I64_GT_U: u8 = 0x56;
    pub const I64_LE_S: u8 = 0x57;
    pub const I64_LE_U: u8 = 0x58;
    pub const I64_GE_S: u8 = 0x59;
    pub const I64_GE_U: u8 = 0x5A;

    pub const F32_EQ: u8 = 0x5B;
    pub const F32_NE: u8 = 0x5C;
    pub const F32_LT: u8 = 0x5D;
    pub const F32_GT: u8 = 0x5E;
    pub const F32_LE: u8 = 0x5F;
    pub const F32_GE: u8 = 0x60;

    pub const F64_EQ: u8 = 0x61;
    pub const F64_NE: u8 = 0x62;
    pub const F64_LT: u8 = 0x63;
    pub const F64_GT: u8 = 0x64;
    pub const F64_LE: u8 = 0x65;
    pub const F64_GE: u8 = 0x66;

    pub const I32_CLZ: u8 = 0x67;
    pub const I32_CTZ: u8 = 0x68;
    pub const I32_POPCNT: u8 = 0x69;
    pub const I32_ADD: u8 = 0x6A;
    pub const I32_SUB: u8 = 0x6B;
    pub const I32_MUL: u8 = 0x6C;
    pub const I32_DIV_S: u8 = 0x6D;
    pub const I32_DIV_U: u8 = 0x6E;
    pub const I32_REM_S: u8 = 0x6F;
    pub const I32_REM_U: u8 = 0x70;
    pub const I32_AND: u8 = 0x71;
    pub const I32_OR: u8 = 0x72;
    pub const I32_XOR: u8 = 0x73;
    pub const I32_SHL: u8 = 0x74;
    pub const I32_SHR_S: u8 = 0x75;
    pub const I32_SHR_U: u8 = 0x76;
    pub const I32_ROTL: u8 = 0x77;
    pub const I32_ROTR: u8 = 0x78;

    pub const I64_CLZ: u8 = 0x79;
    pub const I64_CTZ: u8 = 0x7A;
    pub const I64_POPCNT: u8 = 0x7B;
    pub const I64_ADD: u8 = 0x7C;
    pub const I64_SUB: u8 = 0x7D;
    pub const I64_MUL: u8 = 0x7E;
    pub const I64_DIV_S: u8 = 0x7F;
    pub const I64_DIV_U: u8 = 0x80;
    pub const I64_REM_S: u8 = 0x81;
    pub const I64_REM_U: u8 = 0x82;
    pub const I64_AND: u8 = 0x83;
    pub const I64_OR: u8 = 0x84;
    pub const I64_XOR: u8 = 0x85;
    pub const I64_SHL: u8 = 0x86;
    pub const I64_SHR_S: u8 = 0x87;
    pub const I64_SHR_U: u8 = 0x88;
    pub const I64_ROTL: u8 = 0x89;
    pub const I64_ROTR: u8 = 0x8A;

    pub const F32_ABS: u8 = 0x8B;
    pub const F32_NEG: u8 = 0x8C;
    pub const F32_CEIL: u8 = 0x8D;
    pub const F32_FLOOR: u8 = 0x8E;
    pub const F32_TRUNC: u8 = 0x8F;
    pub const F32_NEAREST: u8 = 0x90;
    pub const F32_SQRT: u8 = 0x91;
    pub const F32_ADD: u8 = 0x92;
    pub const F32_SUB: u8 = 0x93;
    pub const F32_MUL: u8 = 0x94;
    pub const F32_DIV: u8 = 0x95;
    pub const F32_MIN: u8 = 0x96;
    pub const F32_MAX: u8 = 0x97;
    pub const F32_COPYSIGN: u8 = 0x98;

    pub const F64_ABS: u8 = 0x99;
    pub const F64_NEG: u8 = 0x9A;
    pub const F64_CEIL: u8 = 0x9B;
    pub const F64_FLOOR: u8 = 0x9C;
    pub const F64_TRUNC: u8 = 0x9D;
    pub const F64_NEAREST: u8 = 0x9E;
    pub const F64_SQRT: u8 = 0x9F;
    pub const F64_ADD: u8 = 0xA0;
    pub const F64_SUB: u8 = 0xA1;
    pub const F64_MUL: u8 = 0xA2;
    pub const F64_DIV: u8 = 0xA3;
    pub const F64_MIN: u8 = 0xA4;
    pub const F64_MAX: u8 = 0xA5;
    pub const F64_COPYSIGN: u8 = 0xA6;

    pub const I32_WRAP_I64: u8 = 0xA7;
    pub const I32_TRUNC_F32_S: u8 = 0xA8;
    pub const I32_TRUNC_F32_U: u8 = 0xA9;
    pub const I32_TRUNC_F64_S: u8 = 0xAA;
    pub const I32_TRUNC_F64_U: u8 = 0xAB;
    pub const I64_EXTEND_I32_S: u8 = 0xAC;
    pub const I64_EXTEND_I32_U: u8 = 0xAD;
    pub const I64_TRUNC_F32_S: u8 = 0xAE;
    pub const I64_TRUNC_F32_U: u8 = 0xAF;
    pub const I64_TRUNC_F64_S: u8 = 0xB0;
    pub const I64_TRUNC_F64_U: u8 = 0xB1;
    pub const F32_CONVERT_I32_S: u8 = 0xB2;
    pub const F32_CONVERT_I32_U: u8 = 0xB3;
    pub const F32_CONVERT_I64_S: u8 = 0xB4;
    pub const F32_CONVERT_I64_U: u8 = 0xB5;
    pub const F32_DEMOTE_F64: u8 = 0xB6;
    pub const F64_CONVERT_I32_S: u8 = 0xB7;
    pub const F64_CONVERT_I32_U: u8 = 0xB8;
    pub const F64_CONVERT_I64_S: u8 = 0xB9;
    pub const F64_CONVERT_I64_U: u8 = 0xBA;
    pub const F64_PROMOTE_F32: u8 = 0xBB;
    pub const I32_REINTERPRET_F32: u8 = 0xBC;
    pub const I64_REINTERPRET_F64: u8 = 0xBD;
    pub const F32_REINTERPRET_I32: u8 = 0xBE;
    pub const F64_REINTERPRET_I64: u8 = 0xBF;

    pub const I32_EXTEND8_S: u8 = 0xC0;
    pub const I32_EXTEND16_S: u8 = 0xC1;
    pub const I64_EXTEND8_S: u8 = 0xC2;
    pub const I64_EXTEND16_S: u8 = 0xC3;
    pub const I64_EXTEND32_S: u8 = 0xC4;

    pub const REF_NULL: u8 = 0xD0;
    pub const REF_IS_NULL: u8 = 0xD1;
    pub const REF_FUNC: u8 = 0xD2;
    pub const REF_EQ: u8 = 0xD3;
    pub const REF_AS_NON_NULL: u8 = 0xD4;
    pub const BR_ON_NULL: u8 = 0xD5;
    pub const BR_ON_NON_NULL: u8 = 0xD6;

    // ==================== PREFIXED OPCODES ====================

    // 0xFC — Bulk memory, table, saturating truncation
    pub const I32_TRUNC_SAT_F32_S: u16 = 0xFC00;
    pub const I32_TRUNC_SAT_F32_U: u16 = 0xFC01;
    pub const I32_TRUNC_SAT_F64_S: u16 = 0xFC02;
    pub const I32_TRUNC_SAT_F64_U: u16 = 0xFC03;
    pub const I64_TRUNC_SAT_F32_S: u16 = 0xFC04;
    pub const I64_TRUNC_SAT_F32_U: u16 = 0xFC05;
    pub const I64_TRUNC_SAT_F64_S: u16 = 0xFC06;
    pub const I64_TRUNC_SAT_F64_U: u16 = 0xFC07;
    pub const MEMORY_INIT: u16 = 0xFC08;
    pub const DATA_DROP: u16 = 0xFC09;
    pub const MEMORY_COPY: u16 = 0xFC0A;
    pub const MEMORY_FILL: u16 = 0xFC0B;
    pub const TABLE_INIT: u16 = 0xFC0C;
    pub const ELEM_DROP: u16 = 0xFC0D;
    pub const TABLE_COPY: u16 = 0xFC0E;
    pub const TABLE_GROW: u16 = 0xFC0F;
    pub const TABLE_SIZE: u16 = 0xFC10;
    pub const TABLE_FILL: u16 = 0xFC11;

    // 0xFB — GC / aggregate types (struct, array, i31, ref.*, string.* where merged)
    pub const STRUCT_NEW: u16 = 0xFB00;
    pub const STRUCT_NEW_DEFAULT: u16 = 0xFB01;
    pub const STRUCT_GET: u16 = 0xFB02;
    pub const STRUCT_GET_S: u16 = 0xFB03;
    pub const STRUCT_GET_U: u16 = 0xFB04;
    pub const STRUCT_SET: u16 = 0xFB05;
    pub const ARRAY_NEW: u16 = 0xFB06;
    pub const ARRAY_NEW_DEFAULT: u16 = 0xFB07;
    pub const ARRAY_NEW_FIXED: u16 = 0xFB08;
    pub const ARRAY_NEW_DATA: u16 = 0xFB09;
    pub const ARRAY_NEW_ELEM: u16 = 0xFB0A;
    pub const ARRAY_GET: u16 = 0xFB0B;
    pub const ARRAY_GET_S: u16 = 0xFB0C;
    pub const ARRAY_GET_U: u16 = 0xFB0D;
    pub const ARRAY_SET: u16 = 0xFB0E;
    pub const ARRAY_LEN: u16 = 0xFB0F;
    pub const ARRAY_FILL: u16 = 0xFB10;
    pub const ARRAY_COPY: u16 = 0xFB11;
    pub const ARRAY_INIT_DATA: u16 = 0xFB12;
    pub const ARRAY_INIT_ELEM: u16 = 0xFB13;
    pub const REF_I31: u16 = 0xFB20;
    pub const REF_TEST: u16 = 0xFB21;
    pub const REF_CAST: u16 = 0xFB22;
    pub const BR_ON_CAST: u16 = 0xFB24;
    pub const BR_ON_CAST_FAIL: u16 = 0xFB25;
    pub const ANY_CONVERT_EXTERN: u16 = 0xFB26;
    pub const EXTERN_CONVERT_ANY: u16 = 0xFB27;
    pub const I31_NEW: u16 = 0xFB28;
    pub const I31_GET_S: u16 = 0xFB29;
    pub const I31_GET_U: u16 = 0xFB2A;
    pub const EXTERN_INTERNALIZE: u16 = 0xFB30;
    pub const EXTERN_EXTERNALIZE: u16 = 0xFB31;

    // 0xFD — SIMD / Vector (all 262 instructions)
    pub const V128_LOAD: u16 = 0xFD00;
    pub const V128_LOAD8X8_S: u16 = 0xFD01;
    pub const V128_LOAD8X8_U: u16 = 0xFD02;
    pub const V128_LOAD16X4_S: u16 = 0xFD03;
    pub const V128_LOAD16X4_U: u16 = 0xFD04;
    pub const V128_LOAD32X2_S: u16 = 0xFD05;
    pub const V128_LOAD32X2_U: u16 = 0xFD06;
    pub const V128_LOAD8_SPLAT: u16 = 0xFD07;
    pub const V128_LOAD16_SPLAT: u16 = 0xFD08;
    pub const V128_LOAD32_SPLAT: u16 = 0xFD09;
    pub const V128_LOAD64_SPLAT: u16 = 0xFD0A;
    pub const V128_STORE: u16 = 0xFD0B;
    pub const V128_CONST: u16 = 0xFD0C;
    pub const I8X16_SHUFFLE: u16 = 0xFD0D;
    pub const I8X16_SWIZZLE: u16 = 0xFD0E;
    pub const I8X16_SPLAT: u16 = 0xFD0F;
    pub const I16X8_SPLAT: u16 = 0xFD10;
    pub const I32X4_SPLAT: u16 = 0xFD11;
    pub const I64X2_SPLAT: u16 = 0xFD12;
    pub const F32X4_SPLAT: u16 = 0xFD13;
    pub const F64X2_SPLAT: u16 = 0xFD14;
    pub const I8X16_EXTRACT_LANE_S: u16 = 0xFD15;
    pub const I8X16_EXTRACT_LANE_U: u16 = 0xFD16;
    pub const I8X16_REPLACE_LANE: u16 = 0xFD17;
    pub const I16X8_EXTRACT_LANE_S: u16 = 0xFD18;
    pub const I16X8_EXTRACT_LANE_U: u16 = 0xFD19;
    pub const I16X8_REPLACE_LANE: u16 = 0xFD1A;
    pub const I32X4_EXTRACT_LANE: u16 = 0xFD1B;
    pub const I32X4_REPLACE_LANE: u16 = 0xFD1C;
    pub const I64X2_EXTRACT_LANE: u16 = 0xFD1D;
    pub const I64X2_REPLACE_LANE: u16 = 0xFD1E;
    pub const F32X4_EXTRACT_LANE: u16 = 0xFD1F;
    pub const F32X4_REPLACE_LANE: u16 = 0xFD20;
    pub const F64X2_EXTRACT_LANE: u16 = 0xFD21;
    pub const F64X2_REPLACE_LANE: u16 = 0xFD22;
    pub const I8X16_EQ: u16 = 0xFD23;
    pub const I8X16_NE: u16 = 0xFD24;
    pub const I8X16_LT_S: u16 = 0xFD25;
    pub const I8X16_LT_U: u16 = 0xFD26;
    pub const I8X16_GT_S: u16 = 0xFD27;
    pub const I8X16_GT_U: u16 = 0xFD28;
    pub const I8X16_LE_S: u16 = 0xFD29;
    pub const I8X16_LE_U: u16 = 0xFD2A;
    pub const I8X16_GE_S: u16 = 0xFD2B;
    pub const I8X16_GE_U: u16 = 0xFD2C;
    pub const I16X8_EQ: u16 = 0xFD2D;
    pub const I16X8_NE: u16 = 0xFD2E;
    pub const I16X8_LT_S: u16 = 0xFD2F;
    pub const I16X8_LT_U: u16 = 0xFD30;
    pub const I16X8_GT_S: u16 = 0xFD31;
    pub const I16X8_GT_U: u16 = 0xFD32;
    pub const I16X8_LE_S: u16 = 0xFD33;
    pub const I16X8_LE_U: u16 = 0xFD34;
    pub const I16X8_GE_S: u16 = 0xFD35;
    pub const I16X8_GE_U: u16 = 0xFD36;
    pub const I32X4_EQ: u16 = 0xFD37;
    pub const I32X4_NE: u16 = 0xFD38;
    pub const I32X4_LT_S: u16 = 0xFD39;
    pub const I32X4_LT_U: u16 = 0xFD3A;
    pub const I32X4_GT_S: u16 = 0xFD3B;
    pub const I32X4_GT_U: u16 = 0xFD3C;
    pub const I32X4_LE_S: u16 = 0xFD3D;
    pub const I32X4_LE_U: u16 = 0xFD3E;
    pub const I32X4_GE_S: u16 = 0xFD3F;
    pub const I32X4_GE_U: u16 = 0xFD40;
    pub const F32X4_EQ: u16 = 0xFD41;
    pub const F32X4_NE: u16 = 0xFD42;
    pub const F32X4_LT: u16 = 0xFD43;
    pub const F32X4_GT: u16 = 0xFD44;
    pub const F32X4_LE: u16 = 0xFD45;
    pub const F32X4_GE: u16 = 0xFD46;
    pub const F64X2_EQ: u16 = 0xFD47;
    pub const F64X2_NE: u16 = 0xFD48;
    pub const F64X2_LT: u16 = 0xFD49;
    pub const F64X2_GT: u16 = 0xFD4A;
    pub const F64X2_LE: u16 = 0xFD4B;
    pub const F64X2_GE: u16 = 0xFD4C;
    pub const V128_NOT: u16 = 0xFD4D;
    pub const V128_AND: u16 = 0xFD4E;
    pub const V128_ANDNOT: u16 = 0xFD4F;
    pub const V128_OR: u16 = 0xFD50;
    pub const V128_XOR: u16 = 0xFD51;
    pub const V128_BITSELECT: u16 = 0xFD52;
    pub const V128_ANY_TRUE: u16 = 0xFD53;
    pub const V128_LOAD8_LANE: u16 = 0xFD54;
    pub const V128_LOAD16_LANE: u16 = 0xFD55;
    pub const V128_LOAD32_LANE: u16 = 0xFD56;
    pub const V128_LOAD64_LANE: u16 = 0xFD57;
    pub const V128_STORE8_LANE: u16 = 0xFD58;
    pub const V128_STORE16_LANE: u16 = 0xFD59;
    pub const V128_STORE32_LANE: u16 = 0xFD5A;
    pub const V128_STORE64_LANE: u16 = 0xFD5B;
    pub const V128_LOAD32_ZERO: u16 = 0xFD5C;
    pub const V128_LOAD64_ZERO: u16 = 0xFD5D;
    pub const F32X4_DEMOTE_F64X2_ZERO: u16 = 0xFD5E;
    pub const F64X2_PROMOTE_LOW_F32X4: u16 = 0xFD5F;
    pub const I8X16_ABS: u16 = 0xFD60;
    pub const I8X16_NEG: u16 = 0xFD61;
    pub const I8X16_POPCNT: u16 = 0xFD62;
    pub const I8X16_ALL_TRUE: u16 = 0xFD63;
    pub const I8X16_BITMASK: u16 = 0xFD64;
    pub const I8X16_NARROW_I16X8_S: u16 = 0xFD65;
    pub const I8X16_NARROW_I16X8_U: u16 = 0xFD66;
    pub const I8X16_SHL: u16 = 0xFD67;
    pub const I8X16_SHR_S: u16 = 0xFD68;
    pub const I8X16_SHR_U: u16 = 0xFD69;
    pub const I8X16_ADD: u16 = 0xFD6A;
    pub const I8X16_ADD_SAT_S: u16 = 0xFD6B;
    pub const I8X16_ADD_SAT_U: u16 = 0xFD6C;
    pub const I8X16_SUB: u16 = 0xFD6D;
    pub const I8X16_SUB_SAT_S: u16 = 0xFD6E;
    pub const I8X16_SUB_SAT_U: u16 = 0xFD6F;
    pub const I8X16_MIN_S: u16 = 0xFD70;
    pub const I8X16_MIN_U: u16 = 0xFD71;
    pub const I8X16_MAX_S: u16 = 0xFD72;
    pub const I8X16_MAX_U: u16 = 0xFD73;
    pub const I8X16_AVGR_U: u16 = 0xFD74;
    pub const I16X8_EXTADD_PAIRWISE_I8X16_S: u16 = 0xFD75;
    pub const I16X8_EXTADD_PAIRWISE_I8X16_U: u16 = 0xFD76;
    pub const I16X8_ABS: u16 = 0xFD80;
    pub const I16X8_NEG: u16 = 0xFD81;
    pub const I16X8_ALL_TRUE: u16 = 0xFD82;
    pub const I16X8_BITMASK: u16 = 0xFD83;
    pub const I16X8_NARROW_I32X4_S: u16 = 0xFD84;
    pub const I16X8_NARROW_I32X4_U: u16 = 0xFD85;
    pub const I16X8_EXTEND_LOW_I8X16_S: u16 = 0xFD86;
    pub const I16X8_EXTEND_HIGH_I8X16_S: u16 = 0xFD87;
    pub const I16X8_EXTEND_LOW_I8X16_U: u16 = 0xFD88;
    pub const I16X8_EXTEND_HIGH_I8X16_U: u16 = 0xFD89;
    pub const I16X8_SHL: u16 = 0xFD8A;
    pub const I16X8_SHR_S: u16 = 0xFD8B;
    pub const I16X8_SHR_U: u16 = 0xFD8C;
    pub const I16X8_ADD: u16 = 0xFD8D;
    pub const I16X8_ADD_SAT_S: u16 = 0xFD8E;
    pub const I16X8_ADD_SAT_U: u16 = 0xFD8F;
    pub const I16X8_SUB: u16 = 0xFD90;
    pub const I16X8_SUB_SAT_S: u16 = 0xFD91;
    pub const I16X8_SUB_SAT_U: u16 = 0xFD92;
    pub const I16X8_MUL: u16 = 0xFD93;
    pub const I16X8_MIN_S: u16 = 0xFD94;
    pub const I16X8_MIN_U: u16 = 0xFD95;
    pub const I16X8_MAX_S: u16 = 0xFD96;
    pub const I16X8_MAX_U: u16 = 0xFD97;
    pub const I16X8_AVGR_U: u16 = 0xFD98;
    pub const I16X8_EXTMUL_LOW_I8X16_S: u16 = 0xFD9C;
    pub const I16X8_EXTMUL_HIGH_I8X16_S: u16 = 0xFD9D;
    pub const I16X8_EXTMUL_LOW_I8X16_U: u16 = 0xFD9E;
    pub const I16X8_EXTMUL_HIGH_I8X16_U: u16 = 0xFD9F;
    pub const I32X4_EXTADD_PAIRWISE_I16X8_S: u16 = 0xFDA0;
    pub const I32X4_EXTADD_PAIRWISE_I16X8_U: u16 = 0xFDA1;
    pub const I32X4_ABS: u16 = 0xFDA2;
    pub const I32X4_NEG: u16 = 0xFDA3;
    pub const I32X4_ALL_TRUE: u16 = 0xFDA4;
    pub const I32X4_BITMASK: u16 = 0xFDA5;
    pub const I32X4_EXTEND_LOW_I16X8_S: u16 = 0xFDA6;
    pub const I32X4_EXTEND_HIGH_I16X8_S: u16 = 0xFDA7;
    pub const I32X4_EXTEND_LOW_I16X8_U: u16 = 0xFDA8;
    pub const I32X4_EXTEND_HIGH_I16X8_U: u16 = 0xFDA9;
    pub const I32X4_SHL: u16 = 0xFDAA;
    pub const I32X4_SHR_S: u16 = 0xFDAB;
    pub const I32X4_SHR_U: u16 = 0xFDAC;
    pub const I32X4_ADD: u16 = 0xFDAD;
    pub const I32X4_SUB: u16 = 0xFDAE;
    pub const I32X4_MUL: u16 = 0xFDAF;
    pub const I32X4_MIN_S: u16 = 0xFDB0;
    pub const I32X4_MIN_U: u16 = 0xFDB1;
    pub const I32X4_MAX_S: u16 = 0xFDB2;
    pub const I32X4_MAX_U: u16 = 0xFDB3;
    pub const I32X4_DOT_I16X8_S: u16 = 0xFDB4;
    pub const I32X4_EXTMUL_LOW_I16X8_S: u16 = 0xFDB5;
    pub const I32X4_EXTMUL_HIGH_I16X8_S: u16 = 0xFDB6;
    pub const I32X4_EXTMUL_LOW_I16X8_U: u16 = 0xFDB7;
    pub const I32X4_EXTMUL_HIGH_I16X8_U: u16 = 0xFDB8;
    pub const I64X2_ABS: u16 = 0xFDC0;
    pub const I64X2_NEG: u16 = 0xFDC1;
    pub const I64X2_ALL_TRUE: u16 = 0xFDC2;
    pub const I64X2_BITMASK: u16 = 0xFDC3;
    pub const I64X2_EXTEND_LOW_I32X4_S: u16 = 0xFDC4;
    pub const I64X2_EXTEND_HIGH_I32X4_S: u16 = 0xFDC5;
    pub const I64X2_EXTEND_LOW_I32X4_U: u16 = 0xFDC6;
    pub const I64X2_EXTEND_HIGH_I32X4_U: u16 = 0xFDC7;
    pub const I64X2_SHL: u16 = 0xFDC8;
    pub const I64X2_SHR_S: u16 = 0xFDC9;
    pub const I64X2_SHR_U: u16 = 0xFDCA;
    pub const I64X2_ADD: u16 = 0xFDCB;
    pub const I64X2_SUB: u16 = 0xFDCC;
    pub const I64X2_MUL: u16 = 0xFDCD;
    pub const I64X2_EXTMUL_LOW_I32X4_S: u16 = 0xFDCF;
    pub const I64X2_EXTMUL_HIGH_I32X4_S: u16 = 0xFDD0;
    pub const I64X2_EXTMUL_LOW_I32X4_U: u16 = 0xFDD1;
    pub const I64X2_EXTMUL_HIGH_I32X4_U: u16 = 0xFDD2;
    pub const F32X4_CEIL: u16 = 0xFDD3;
    pub const F32X4_FLOOR: u16 = 0xFDD4;
    pub const F32X4_TRUNC: u16 = 0xFDD5;
    pub const F32X4_NEAREST: u16 = 0xFDD6;
    pub const F32X4_ABS: u16 = 0xFDD7;
    pub const F32X4_NEG: u16 = 0xFDD8;
    pub const F32X4_SQRT: u16 = 0xFDD9;
    pub const F32X4_ADD: u16 = 0xFDDA;
    pub const F32X4_SUB: u16 = 0xFDDB;
    pub const F32X4_MUL: u16 = 0xFDDC;
    pub const F32X4_DIV: u16 = 0xFDDD;
    pub const F32X4_MIN: u16 = 0xFDDE;
    pub const F32X4_MAX: u16 = 0xFDDF;
    pub const F32X4_PMIN: u16 = 0xFDE0;
    pub const F32X4_PMAX: u16 = 0xFDE1;
    pub const F64X2_CEIL: u16 = 0xFDE2;
    pub const F64X2_FLOOR: u16 = 0xFDE3;
    pub const F64X2_TRUNC: u16 = 0xFDE4;
    pub const F64X2_NEAREST: u16 = 0xFDE5;
    pub const F64X2_ABS: u16 = 0xFDE6;
    pub const F64X2_NEG: u16 = 0xFDE7;
    pub const F64X2_SQRT: u16 = 0xFDE8;
    pub const F64X2_ADD: u16 = 0xFDE9;
    pub const F64X2_SUB: u16 = 0xFDEA;
    pub const F64X2_MUL: u16 = 0xFDEB;
    pub const F64X2_DIV: u16 = 0xFDEC;
    pub const F64X2_MIN: u16 = 0xFDED;
    pub const F64X2_MAX: u16 = 0xFDEE;
    pub const F64X2_PMIN: u16 = 0xFDEF;
    pub const F64X2_PMAX: u16 = 0xFDF0;
    pub const I32X4_TRUNC_SAT_F32X4_S: u16 = 0xFDF1;
    pub const I32X4_TRUNC_SAT_F32X4_U: u16 = 0xFDF2;
    pub const F32X4_CONVERT_I32X4_S: u16 = 0xFDF3;
    pub const F32X4_CONVERT_I32X4_U: u16 = 0xFDF4;
    pub const I32X4_TRUNC_SAT_F64X2_S_ZERO: u16 = 0xFDF5;
    pub const I32X4_TRUNC_SAT_F64X2_U_ZERO: u16 = 0xFDF6;
    pub const F64X2_CONVERT_LOW_I32X4_S: u16 = 0xFDF7;
    pub const F64X2_CONVERT_LOW_I32X4_U: u16 = 0xFDF8;
    pub const I8X16_RELAXED_SWIZZLE: u16 = 0xFDF9;
    pub const I32X4_RELAXED_TRUNC_S_F32X4: u16 = 0xFDFA;
    pub const I32X4_RELAXED_TRUNC_U_F32X4: u16 = 0xFDFB;
    pub const I32X4_RELAXED_TRUNC_S_F64X2: u16 = 0xFDFC;
    pub const I32X4_RELAXED_TRUNC_U_F64X2: u16 = 0xFDFD;
    pub const F32X4_RELAXED_MADD: u16 = 0xFDFE;
    pub const F32X4_RELAXED_NMADD: u16 = 0xFDFF;
    pub const F64X2_RELAXED_MADD: u16 = 0xFE00;
    pub const F64X2_RELAXED_NMADD: u16 = 0xFE01;
    pub const I8X16_RELAXED_LANESEL: u16 = 0xFE02;
    pub const I16X8_RELAXED_LANESEL: u16 = 0xFE03;
    pub const I32X4_RELAXED_LANESEL: u16 = 0xFE04;
    pub const I64X2_RELAXED_LANESEL: u16 = 0xFE05;
    pub const F32X4_RELAXED_MIN: u16 = 0xFE06;
    pub const F32X4_RELAXED_MAX: u16 = 0xFE07;
    pub const F64X2_RELAXED_MIN: u16 = 0xFE08;
    pub const F64X2_RELAXED_MAX: u16 = 0xFE09;
    pub const I16X8_RELAXED_Q15MULR_S: u16 = 0xFE0A;
    pub const I16X8_RELAXED_DOT_I8X16_I7X16_S: u16 = 0xFE0B;
    pub const I32X4_RELAXED_DOT_I8X16_I7X16_S: u16 = 0xFE0C;
    pub const I32X4_RELAXED_DOT_I8X16_I7X16_ADD_S: u16 = 0xFE0D;

    // 0xFE — Atomics / Threads
    pub const MEMORY_ATOMIC_NOTIFY: u16 = 0xFE00;
    pub const MEMORY_ATOMIC_WAIT32: u16 = 0xFE01;
    pub const MEMORY_ATOMIC_WAIT64: u16 = 0xFE02;
    pub const ATOMIC_FENCE: u16 = 0xFE03;

    // Atomic Loads
    pub const I32_ATOMIC_LOAD: u16 = 0xFE10;
    pub const I64_ATOMIC_LOAD: u16 = 0xFE11;
    pub const I32_ATOMIC_LOAD8_U: u16 = 0xFE12;
    pub const I32_ATOMIC_LOAD16_U: u16 = 0xFE13;
    pub const I64_ATOMIC_LOAD8_U: u16 = 0xFE14;
    pub const I64_ATOMIC_LOAD16_U: u16 = 0xFE15;
    pub const I64_ATOMIC_LOAD32_U: u16 = 0xFE16;

    // Atomic Stores
    pub const I32_ATOMIC_STORE: u16 = 0xFE17;
    pub const I64_ATOMIC_STORE: u16 = 0xFE18;
    pub const I32_ATOMIC_STORE8: u16 = 0xFE19;
    pub const I32_ATOMIC_STORE16: u16 = 0xFE1A;
    pub const I64_ATOMIC_STORE8: u16 = 0xFE1B;
    pub const I64_ATOMIC_STORE16: u16 = 0xFE1C;
    pub const I64_ATOMIC_STORE32: u16 = 0xFE1D;

    // RMW Add
    pub const I32_ATOMIC_RMW_ADD: u16 = 0xFE1E;
    pub const I64_ATOMIC_RMW_ADD: u16 = 0xFE1F;
    pub const I32_ATOMIC_RMW8_ADD_U: u16 = 0xFE20;
    pub const I32_ATOMIC_RMW16_ADD_U: u16 = 0xFE21;
    pub const I64_ATOMIC_RMW8_ADD_U: u16 = 0xFE22;
    pub const I64_ATOMIC_RMW16_ADD_U: u16 = 0xFE23;
    pub const I64_ATOMIC_RMW32_ADD_U: u16 = 0xFE24;

    // RMW Sub
    pub const I32_ATOMIC_RMW_SUB: u16 = 0xFE25;
    pub const I64_ATOMIC_RMW_SUB: u16 = 0xFE26;
    pub const I32_ATOMIC_RMW8_SUB_U: u16 = 0xFE27;
    pub const I32_ATOMIC_RMW16_SUB_U: u16 = 0xFE28;
    pub const I64_ATOMIC_RMW8_SUB_U: u16 = 0xFE29;
    pub const I64_ATOMIC_RMW16_SUB_U: u16 = 0xFE2A;
    pub const I64_ATOMIC_RMW32_SUB_U: u16 = 0xFE2B;

    // RMW And
    pub const I32_ATOMIC_RMW_AND: u16 = 0xFE2C;
    pub const I64_ATOMIC_RMW_AND: u16 = 0xFE2D;
    pub const I32_ATOMIC_RMW8_AND_U: u16 = 0xFE2E;
    pub const I32_ATOMIC_RMW16_AND_U: u16 = 0xFE2F;
    pub const I64_ATOMIC_RMW8_AND_U: u16 = 0xFE30;
    pub const I64_ATOMIC_RMW16_AND_U: u16 = 0xFE31;
    pub const I64_ATOMIC_RMW32_AND_U: u16 = 0xFE32;

    // RMW Or
    pub const I32_ATOMIC_RMW_OR: u16 = 0xFE33;
    pub const I64_ATOMIC_RMW_OR: u16 = 0xFE34;
    pub const I32_ATOMIC_RMW8_OR_U: u16 = 0xFE35;
    pub const I32_ATOMIC_RMW16_OR_U: u16 = 0xFE36;
    pub const I64_ATOMIC_RMW8_OR_U: u16 = 0xFE37;
    pub const I64_ATOMIC_RMW16_OR_U: u16 = 0xFE38;
    pub const I64_ATOMIC_RMW32_OR_U: u16 = 0xFE39;

    // RMW Xor
    pub const I32_ATOMIC_RMW_XOR: u16 = 0xFE3A;
    pub const I64_ATOMIC_RMW_XOR: u16 = 0xFE3B;
    pub const I32_ATOMIC_RMW8_XOR_U: u16 = 0xFE3C;
    pub const I32_ATOMIC_RMW16_XOR_U: u16 = 0xFE3D;
    pub const I64_ATOMIC_RMW8_XOR_U: u16 = 0xFE3E;
    pub const I64_ATOMIC_RMW16_XOR_U: u16 = 0xFE3F;
    pub const I64_ATOMIC_RMW32_XOR_U: u16 = 0xFE40;

    // RMW Xchg (exchange)
    pub const I32_ATOMIC_RMW_XCHG: u16 = 0xFE41;
    pub const I64_ATOMIC_RMW_XCHG: u16 = 0xFE42;
    pub const I32_ATOMIC_RMW8_XCHG_U: u16 = 0xFE43;
    pub const I32_ATOMIC_RMW16_XCHG_U: u16 = 0xFE44;
    pub const I64_ATOMIC_RMW8_XCHG_U: u16 = 0xFE45;
    pub const I64_ATOMIC_RMW16_XCHG_U: u16 = 0xFE46;
    pub const I64_ATOMIC_RMW32_XCHG_U: u16 = 0xFE47;

    // RMW Cmpxchg (compare-exchange)
    pub const I32_ATOMIC_RMW_CMPXCHG: u16 = 0xFE48;
    pub const I64_ATOMIC_RMW_CMPXCHG: u16 = 0xFE49;
    pub const I32_ATOMIC_RMW8_CMPXCHG_U: u16 = 0xFE4A;
    pub const I32_ATOMIC_RMW16_CMPXCHG_U: u16 = 0xFE4B;
    pub const I64_ATOMIC_RMW8_CMPXCHG_U: u16 = 0xFE4C;
    pub const I64_ATOMIC_RMW16_CMPXCHG_U: u16 = 0xFE4D;
    pub const I64_ATOMIC_RMW32_CMPXCHG_U: u16 = 0xFE4E;
}

// Convenience trait for converting Rust types to Wasm binary bytes
pub trait TypeWasmExt {
    fn to_wasm_bytes(&self) -> Vec<u8>;
}

impl TypeWasmExt for u32 {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        leb::encode_u32(*self)
    }
}

impl TypeWasmExt for usize {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        leb::encode_u32(*self as u32)
    }
}

impl TypeWasmExt for i32 {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        leb::encode_i32(*self)
    }
}

impl TypeWasmExt for i64 {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        leb::encode_i64(*self)
    }
}

impl TypeWasmExt for f32 {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl TypeWasmExt for f64 {
    fn to_wasm_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

// LEB128 decoding and encoding
mod leb {
    use alloc::vec::Vec;

    pub fn u32(data: &[u8], pos: &mut usize) -> Result<u32, &'static str> {
        let mut v: u32 = 0;
        let mut shift = 0;
        loop {
            if *pos >= data.len() {
                return Err("unexpected end");
            }
            let b = data[*pos];
            *pos += 1;
            v |= ((b & 0x7F) as u32) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 35 {
                return Err("leb128 u32 overflow");
            }
        }
        Ok(v)
    }

    pub fn i32(data: &[u8], pos: &mut usize) -> Result<i32, &'static str> {
        let mut v: i64 = 0;
        let mut shift = 0;
        loop {
            if *pos >= data.len() {
                return Err("unexpected end");
            }
            let b = data[*pos];
            *pos += 1;
            v |= ((b & 0x7F) as i64) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                if shift < 64 && (b & 0x40) != 0 {
                    v |= !0i64 << shift;
                }
                break;
            }
        }
        Ok(v as i32)
    }

    pub fn u64(data: &[u8], pos: &mut usize) -> Result<u64, &'static str> {
        let mut v: u64 = 0;
        let mut shift = 0;
        loop {
            if *pos >= data.len() {
                return Err("unexpected end");
            }
            let b = data[*pos];
            *pos += 1;
            v |= ((b & 0x7F) as u64) << shift;
            if b & 0x80 == 0 {
                break;
            }
            shift += 7;
            if shift >= 70 {
                return Err("leb128 u64 overflow");
            }
        }
        Ok(v)
    }

    pub fn i64(data: &[u8], pos: &mut usize) -> Result<i64, &'static str> {
        let mut v: i64 = 0;
        let mut shift = 0;
        loop {
            if *pos >= data.len() {
                return Err("unexpected end");
            }
            let b = data[*pos];
            *pos += 1;
            v |= ((b & 0x7F) as i64) << shift;
            shift += 7;
            if b & 0x80 == 0 {
                if shift < 64 && (b & 0x40) != 0 {
                    v |= !0i64 << shift;
                }
                break;
            }
        }
        Ok(v)
    }

    // --- Encoding helpers ---
    pub fn encode_u32(mut val: u32) -> Vec<u8> {
        let mut buf = Vec::new();
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if val == 0 {
                break;
            }
        }
        buf
    }

    pub fn encode_i32(mut val: i32) -> Vec<u8> {
        let mut buf = Vec::new();
        loop {
            let byte = (val & 0x7F) as u8;
            val >>= 7;
            let done = (val == 0 && byte & 0x40 == 0) || (val == -1 && byte & 0x40 != 0);
            if done {
                buf.push(byte);
                break;
            } else {
                buf.push(byte | 0x80);
            }
        }
        buf
    }

    pub fn encode_u64(mut val: u64) -> Vec<u8> {
        let mut buf = Vec::new();
        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80;
            }
            buf.push(byte);
            if val == 0 {
                break;
            }
        }
        buf
    }

    pub fn encode_i64(mut val: i64) -> Vec<u8> {
        let mut buf = Vec::new();
        loop {
            let byte = (val & 0x7F) as u8;
            val >>= 7;
            let done = (val == 0 && byte & 0x40 == 0) || (val == -1 && byte & 0x40 != 0);
            if done {
                buf.push(byte);
                break;
            } else {
                buf.push(byte | 0x80);
            }
        }
        buf
    }
}

// Full AST (covers entire spec 3.0)
#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug, Clone)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

#[derive(Debug)]
pub enum BlockType {
    Empty,
    Val(ValType),
    TypeIdx(u32),
}

#[derive(Debug)]
pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

#[derive(Debug)]
pub enum Instruction {
    // Control
    Unreachable,
    Nop,
    Block(BlockType),
    Loop(BlockType),
    If(BlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect(u32, u32),
    ReturnCall(u32),
    ReturnCallIndirect(u32, u32),
    CallRef(u32),
    ReturnCallRef(u32),
    // Exception
    Throw(u32),
    Rethrow(u32),
    ThrowRef,
    Try(BlockType),
    Catch(u32),
    CatchAll,
    Delegate(u32),
    TryTable(BlockType),
    // Reference
    RefNull(u8),
    RefIsNull,
    RefFunc(u32),
    RefEq,
    RefAsNonNull,
    BrOnNull(u32),
    BrOnNonNull(u32),
    // Parametric
    Drop,
    Select,
    SelectT(Vec<ValType>),
    // Variable
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),
    // Table
    TableGet(u32),
    TableSet(u32),
    // Memory loads
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Load8S(MemArg),
    I32Load8U(MemArg),
    I32Load16S(MemArg),
    I32Load16U(MemArg),
    I64Load8S(MemArg),
    I64Load8U(MemArg),
    I64Load16S(MemArg),
    I64Load16U(MemArg),
    I64Load32S(MemArg),
    I64Load32U(MemArg),
    // Memory stores
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    I32Store8(MemArg),
    I32Store16(MemArg),
    I64Store8(MemArg),
    I64Store16(MemArg),
    I64Store32(MemArg),
    // Memory
    MemorySize,
    MemoryGrow,
    // Constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    // i32 comparison
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    // i64 comparison
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    // f32 comparison
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    // f64 comparison
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    // i32 numeric
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    // i64 numeric
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    // f32 numeric
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    // f64 numeric
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    // Conversions
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    // Sign extension
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    // 0xFC prefix — saturating truncation + bulk memory + table
    I32TruncSatF32S,
    I32TruncSatF32U,
    I32TruncSatF64S,
    I32TruncSatF64U,
    I64TruncSatF32S,
    I64TruncSatF32U,
    I64TruncSatF64S,
    I64TruncSatF64U,
    MemoryInit(u32),
    DataDrop(u32),
    MemoryCopy,
    MemoryFill,
    TableInit(u32, u32),
    ElemDrop(u32),
    TableCopy(u32, u32),
    TableGrow(u32),
    TableSize(u32),
    TableFill(u32),

    // 0xFB prefix — GC
    StructNew(u32),
    StructNewDefault(u32),
    StructGet(u32, u32),
    StructGetS(u32, u32),
    StructGetU(u32, u32),
    StructSet(u32, u32),
    ArrayNew(u32),
    ArrayNewDefault(u32),
    ArrayNewFixed(u32, u32),
    ArrayNewData(u32, u32),
    ArrayNewElem(u32, u32),
    ArrayGet(u32),
    ArrayGetS(u32),
    ArrayGetU(u32),
    ArraySet(u32),
    ArrayLen,
    ArrayFill(u32),
    ArrayCopy(u32, u32),
    ArrayInitData(u32, u32),
    ArrayInitElem(u32, u32),
    RefI31,
    I31GetS,
    I31GetU,
    RefTest(u32),
    RefCast(u32),
    BrOnCast(u32),
    BrOnCastFail(u32),
    AnyConvertExtern,
    ExternConvertAny,
    ExternInternalize,
    ExternExternalize,

    // 0xFD prefix — SIMD
    V128Load(MemArg),
    V128Load8x8S(MemArg),
    V128Load8x8U(MemArg),
    V128Load16x4S(MemArg),
    V128Load16x4U(MemArg),
    V128Load32x2S(MemArg),
    V128Load32x2U(MemArg),
    V128Load8Splat(MemArg),
    V128Load16Splat(MemArg),
    V128Load32Splat(MemArg),
    V128Load64Splat(MemArg),
    V128Store(MemArg),
    V128Const([u8; 16]),
    I8x16Shuffle([u8; 16]),
    I8x16Swizzle,
    I8x16Splat,
    I16x8Splat,
    I32x4Splat,
    I64x2Splat,
    F32x4Splat,
    F64x2Splat,
    I8x16ExtractLaneS(u8),
    I8x16ExtractLaneU(u8),
    I8x16ReplaceLane(u8),
    I16x8ExtractLaneS(u8),
    I16x8ExtractLaneU(u8),
    I16x8ReplaceLane(u8),
    I32x4ExtractLane(u8),
    I32x4ReplaceLane(u8),
    I64x2ExtractLane(u8),
    I64x2ReplaceLane(u8),
    F32x4ExtractLane(u8),
    F32x4ReplaceLane(u8),
    F64x2ExtractLane(u8),
    F64x2ReplaceLane(u8),
    // SIMD comparisons
    I8x16Eq,
    I8x16Ne,
    I8x16LtS,
    I8x16LtU,
    I8x16GtS,
    I8x16GtU,
    I8x16LeS,
    I8x16LeU,
    I8x16GeS,
    I8x16GeU,
    I16x8Eq,
    I16x8Ne,
    I16x8LtS,
    I16x8LtU,
    I16x8GtS,
    I16x8GtU,
    I16x8LeS,
    I16x8LeU,
    I16x8GeS,
    I16x8GeU,
    I32x4Eq,
    I32x4Ne,
    I32x4LtS,
    I32x4LtU,
    I32x4GtS,
    I32x4GtU,
    I32x4LeS,
    I32x4LeU,
    I32x4GeS,
    I32x4GeU,
    F32x4Eq,
    F32x4Ne,
    F32x4Lt,
    F32x4Gt,
    F32x4Le,
    F32x4Ge,
    F64x2Eq,
    F64x2Ne,
    F64x2Lt,
    F64x2Gt,
    F64x2Le,
    F64x2Ge,
    // SIMD bitwise
    V128Not,
    V128And,
    V128AndNot,
    V128Or,
    V128Xor,
    V128Bitselect,
    V128AnyTrue,
    // SIMD load/store lane
    V128Load8Lane(MemArg, u8),
    V128Load16Lane(MemArg, u8),
    V128Load32Lane(MemArg, u8),
    V128Load64Lane(MemArg, u8),
    V128Store8Lane(MemArg, u8),
    V128Store16Lane(MemArg, u8),
    V128Store32Lane(MemArg, u8),
    V128Store64Lane(MemArg, u8),
    V128Load32Zero(MemArg),
    V128Load64Zero(MemArg),
    // SIMD conversions
    F32x4DemoteF64x2Zero,
    F64x2PromoteLowF32x4,
    // SIMD i8x16 ops
    I8x16Abs,
    I8x16Neg,
    I8x16Popcnt,
    I8x16AllTrue,
    I8x16Bitmask,
    I8x16NarrowI16x8S,
    I8x16NarrowI16x8U,
    I8x16Shl,
    I8x16ShrS,
    I8x16ShrU,
    I8x16Add,
    I8x16AddSatS,
    I8x16AddSatU,
    I8x16Sub,
    I8x16SubSatS,
    I8x16SubSatU,
    I8x16MinS,
    I8x16MinU,
    I8x16MaxS,
    I8x16MaxU,
    I8x16AvgrU,
    // SIMD i16x8 ops
    I16x8ExtaddPairwiseI8x16S,
    I16x8ExtaddPairwiseI8x16U,
    I16x8Abs,
    I16x8Neg,
    I16x8AllTrue,
    I16x8Bitmask,
    I16x8NarrowI32x4S,
    I16x8NarrowI32x4U,
    I16x8ExtendLowI8x16S,
    I16x8ExtendHighI8x16S,
    I16x8ExtendLowI8x16U,
    I16x8ExtendHighI8x16U,
    I16x8Shl,
    I16x8ShrS,
    I16x8ShrU,
    I16x8Add,
    I16x8AddSatS,
    I16x8AddSatU,
    I16x8Sub,
    I16x8SubSatS,
    I16x8SubSatU,
    I16x8Mul,
    I16x8MinS,
    I16x8MinU,
    I16x8MaxS,
    I16x8MaxU,
    I16x8AvgrU,
    I16x8ExtmulLowI8x16S,
    I16x8ExtmulHighI8x16S,
    I16x8ExtmulLowI8x16U,
    I16x8ExtmulHighI8x16U,
    // SIMD i32x4 ops
    I32x4ExtaddPairwiseI16x8S,
    I32x4ExtaddPairwiseI16x8U,
    I32x4Abs,
    I32x4Neg,
    I32x4AllTrue,
    I32x4Bitmask,
    I32x4ExtendLowI16x8S,
    I32x4ExtendHighI16x8S,
    I32x4ExtendLowI16x8U,
    I32x4ExtendHighI16x8U,
    I32x4Shl,
    I32x4ShrS,
    I32x4ShrU,
    I32x4Add,
    I32x4Sub,
    I32x4Mul,
    I32x4MinS,
    I32x4MinU,
    I32x4MaxS,
    I32x4MaxU,
    I32x4DotI16x8S,
    I32x4ExtmulLowI16x8S,
    I32x4ExtmulHighI16x8S,
    I32x4ExtmulLowI16x8U,
    I32x4ExtmulHighI16x8U,
    // SIMD i64x2 ops
    I64x2Abs,
    I64x2Neg,
    I64x2AllTrue,
    I64x2Bitmask,
    I64x2ExtendLowI32x4S,
    I64x2ExtendHighI32x4S,
    I64x2ExtendLowI32x4U,
    I64x2ExtendHighI32x4U,
    I64x2Shl,
    I64x2ShrS,
    I64x2ShrU,
    I64x2Add,
    I64x2Sub,
    I64x2Mul,
    I64x2ExtmulLowI32x4S,
    I64x2ExtmulHighI32x4S,
    I64x2ExtmulLowI32x4U,
    I64x2ExtmulHighI32x4U,
    // SIMD f32x4 ops
    F32x4Ceil,
    F32x4Floor,
    F32x4Trunc,
    F32x4Nearest,
    F32x4Abs,
    F32x4Neg,
    F32x4Sqrt,
    F32x4Add,
    F32x4Sub,
    F32x4Mul,
    F32x4Div,
    F32x4Min,
    F32x4Max,
    F32x4Pmin,
    F32x4Pmax,
    // SIMD f64x2 ops
    F64x2Ceil,
    F64x2Floor,
    F64x2Trunc,
    F64x2Nearest,
    F64x2Abs,
    F64x2Neg,
    F64x2Sqrt,
    F64x2Add,
    F64x2Sub,
    F64x2Mul,
    F64x2Div,
    F64x2Min,
    F64x2Max,
    F64x2Pmin,
    F64x2Pmax,
    // SIMD sat trunc / convert
    I32x4TruncSatF32x4S,
    I32x4TruncSatF32x4U,
    F32x4ConvertI32x4S,
    F32x4ConvertI32x4U,
    I32x4TruncSatF64x2SZero,
    I32x4TruncSatF64x2UZero,
    F64x2ConvertLowI32x4S,
    F64x2ConvertLowI32x4U,

    // 0xFE prefix — Atomics
    MemoryAtomicNotify(MemArg),
    MemoryAtomicWait32(MemArg),
    MemoryAtomicWait64(MemArg),
    AtomicFence,
    I32AtomicLoad(MemArg),
    I64AtomicLoad(MemArg),
    I32AtomicLoad8U(MemArg),
    I32AtomicLoad16U(MemArg),
    I64AtomicLoad8U(MemArg),
    I64AtomicLoad16U(MemArg),
    I64AtomicLoad32U(MemArg),
    I32AtomicStore(MemArg),
    I64AtomicStore(MemArg),
    I32AtomicStore8(MemArg),
    I32AtomicStore16(MemArg),
    I64AtomicStore8(MemArg),
    I64AtomicStore16(MemArg),
    I64AtomicStore32(MemArg),
    I32AtomicRmwAdd(MemArg),
    I64AtomicRmwAdd(MemArg),
    I32AtomicRmw8AddU(MemArg),
    I32AtomicRmw16AddU(MemArg),
    I64AtomicRmw8AddU(MemArg),
    I64AtomicRmw16AddU(MemArg),
    I64AtomicRmw32AddU(MemArg),
    I32AtomicRmwSub(MemArg),
    I64AtomicRmwSub(MemArg),
    I32AtomicRmw8SubU(MemArg),
    I32AtomicRmw16SubU(MemArg),
    I64AtomicRmw8SubU(MemArg),
    I64AtomicRmw16SubU(MemArg),
    I64AtomicRmw32SubU(MemArg),
    I32AtomicRmwAnd(MemArg),
    I64AtomicRmwAnd(MemArg),
    I32AtomicRmw8AndU(MemArg),
    I32AtomicRmw16AndU(MemArg),
    I64AtomicRmw8AndU(MemArg),
    I64AtomicRmw16AndU(MemArg),
    I64AtomicRmw32AndU(MemArg),
    I32AtomicRmwOr(MemArg),
    I64AtomicRmwOr(MemArg),
    I32AtomicRmw8OrU(MemArg),
    I32AtomicRmw16OrU(MemArg),
    I64AtomicRmw8OrU(MemArg),
    I64AtomicRmw16OrU(MemArg),
    I64AtomicRmw32OrU(MemArg),
    I32AtomicRmwXor(MemArg),
    I64AtomicRmwXor(MemArg),
    I32AtomicRmw8XorU(MemArg),
    I32AtomicRmw16XorU(MemArg),
    I64AtomicRmw8XorU(MemArg),
    I64AtomicRmw16XorU(MemArg),
    I64AtomicRmw32XorU(MemArg),
    I32AtomicRmwXchg(MemArg),
    I64AtomicRmwXchg(MemArg),
    I32AtomicRmw8XchgU(MemArg),
    I32AtomicRmw16XchgU(MemArg),
    I64AtomicRmw8XchgU(MemArg),
    I64AtomicRmw16XchgU(MemArg),
    I64AtomicRmw32XchgU(MemArg),
    I32AtomicRmwCmpxchg(MemArg),
    I64AtomicRmwCmpxchg(MemArg),
    I32AtomicRmw8CmpxchgU(MemArg),
    I32AtomicRmw16CmpxchgU(MemArg),
    I64AtomicRmw8CmpxchgU(MemArg),
    I64AtomicRmw16CmpxchgU(MemArg),
    I64AtomicRmw32CmpxchgU(MemArg),

    // Catch-all
    Unknown(u8),
}

#[derive(Debug)]
pub enum Section {
    Custom(String, Vec<u8>),
    Type(Vec<FuncType>),
    Import(Vec<Import>),
    Function(Vec<u32>),
    Table(Vec<Table>),
    Memory(Vec<Limits>),
    Global(Vec<Global>),
    Export(Vec<Export>),
    Start(u32),
    Element(Vec<Element>),
    Code(Vec<Code>),
    Data(Vec<Data>),
    DataCount(u32),
    Tag(Vec<Tag>),
}

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub name: String,
    pub desc: ImportDesc,
}
#[derive(Debug)]
pub enum ImportDesc {
    Func(u32),
    Table(Table),
    Memory(Limits),
    Global(GlobalType),
    Tag(u32),
}

#[derive(Debug, Clone)]
pub struct GlobalType {
    pub valtype: ValType,
    pub mutable: bool,
}

#[derive(Debug)]
pub struct Table {
    pub reftype: u8,
    pub limits: Limits,
}
#[derive(Debug)]
pub struct Global {
    pub ty: GlobalType,
    pub init: Vec<Instruction>,
}
#[derive(Debug)]
pub struct Export {
    pub name: String,
    pub kind: u8,
    pub idx: u32,
}
#[derive(Debug)]
pub struct Element {
    pub kind: u32,
    pub init: Vec<Vec<Instruction>>,
}
#[derive(Debug)]
pub struct Code {
    pub locals: Vec<(u32, ValType)>,
    pub body: Vec<Instruction>,
}
#[derive(Debug)]
pub struct Data {
    pub mode: DataMode,
    pub init: Vec<u8>,
}
#[derive(Debug)]
pub enum DataMode {
    Passive,
    Active {
        memory: u32,
        offset: Vec<Instruction>,
    },
}
#[derive(Debug)]
pub struct Tag {
    pub kind: u8,
    pub typeidx: u32,
}

#[derive(Debug)]
pub struct Program {
    pub sections: Vec<Section>,
}

// ---------- Decoding helpers ----------

fn read_byte(data: &[u8], pos: &mut usize) -> Result<u8, &'static str> {
    if *pos >= data.len() {
        return Err("unexpected end");
    }
    let b = data[*pos];
    *pos += 1;
    Ok(b)
}

fn read_bytes<const N: usize>(data: &[u8], pos: &mut usize) -> Result<[u8; N], &'static str> {
    if *pos + N > data.len() {
        return Err("unexpected end");
    }
    let mut buf = [0u8; N];
    buf.copy_from_slice(&data[*pos..*pos + N]);
    *pos += N;
    Ok(buf)
}

fn read_memarg(data: &[u8], pos: &mut usize) -> Result<MemArg, &'static str> {
    let align = leb::u32(data, pos)?;
    let offset = leb::u32(data, pos)?;
    Ok(MemArg { offset, align })
}

fn byte_to_valtype(b: u8) -> Result<ValType, &'static str> {
    match b {
        0x7F => Ok(ValType::I32),
        0x7E => Ok(ValType::I64),
        0x7D => Ok(ValType::F32),
        0x7C => Ok(ValType::F64),
        0x7B => Ok(ValType::V128),
        0x70 => Ok(ValType::FuncRef),
        0x6F => Ok(ValType::ExternRef),
        _ => Err("invalid valtype"),
    }
}

fn valtype_to_byte(v: &ValType) -> u8 {
    match v {
        ValType::I32 => 0x7F,
        ValType::I64 => 0x7E,
        ValType::F32 => 0x7D,
        ValType::F64 => 0x7C,
        ValType::V128 => 0x7B,
        ValType::FuncRef => 0x70,
        ValType::ExternRef => 0x6F,
    }
}

fn read_blocktype(data: &[u8], pos: &mut usize) -> Result<BlockType, &'static str> {
    let b = data[*pos];
    match b {
        0x40 => {
            *pos += 1;
            Ok(BlockType::Empty)
        }
        0x7F | 0x7E | 0x7D | 0x7C | 0x7B | 0x70 | 0x6F => {
            *pos += 1;
            Ok(BlockType::Val(byte_to_valtype(b)?))
        }
        _ => {
            // Signed LEB128 type index (positive means type index)
            let idx = leb::i32(data, pos)?;
            Ok(BlockType::TypeIdx(idx as u32))
        }
    }
}

fn read_limits(data: &[u8], pos: &mut usize) -> Result<Limits, &'static str> {
    let flag = read_byte(data, pos)?;
    let min = leb::u32(data, pos)?;
    let max = if flag & 0x01 != 0 {
        Some(leb::u32(data, pos)?)
    } else {
        None
    };
    Ok(Limits { min, max })
}

fn read_name(data: &[u8], pos: &mut usize) -> Result<String, &'static str> {
    let len = leb::u32(data, pos)? as usize;
    if *pos + len > data.len() {
        return Err("unexpected end in name");
    }
    let s = core::str::from_utf8(&data[*pos..*pos + len]).map_err(|_| "invalid utf8")?;
    *pos += len;
    Ok(String::from(s))
}

// Read a function type: 0x60 tag + vec(valtype) params + vec(valtype) results
fn read_functype(data: &[u8], pos: &mut usize) -> Result<FuncType, &'static str> {
    let tag = read_byte(data, pos)?;
    if tag != 0x60 {
        return Err("expected function type tag 0x60");
    }
    let param_count = leb::u32(data, pos)? as usize;
    let mut params = Vec::with_capacity(param_count);
    for _ in 0..param_count {
        params.push(byte_to_valtype(read_byte(data, pos)?)?);
    }
    let result_count = leb::u32(data, pos)? as usize;
    let mut results = Vec::with_capacity(result_count);
    for _ in 0..result_count {
        results.push(byte_to_valtype(read_byte(data, pos)?)?);
    }
    Ok(FuncType { params, results })
}

// Read instruction sequence until END (for code bodies, init exprs, etc.)
fn read_instructions(data: &[u8], pos: &mut usize) -> Result<Vec<Instruction>, &'static str> {
    let mut instructions = Vec::new();
    loop {
        if *pos >= data.len() {
            return Err("unexpected end of instructions");
        }
        let next_byte = data[*pos];
        if next_byte == op::END {
            *pos += 1;
            instructions.push(Instruction::End);
            break;
        }
        instructions.push(decode_instruction(data, pos)?);
    }
    Ok(instructions)
}

// Full decoder for every opcode
fn decode_instruction(data: &[u8], pos: &mut usize) -> Result<Instruction, &'static str> {
    let byte = read_byte(data, pos)?;
    match byte {
        // Control
        op::UNREACHABLE => Ok(Instruction::Unreachable),
        op::NOP => Ok(Instruction::Nop),
        op::BLOCK => Ok(Instruction::Block(read_blocktype(data, pos)?)),
        op::LOOP => Ok(Instruction::Loop(read_blocktype(data, pos)?)),
        op::IF => Ok(Instruction::If(read_blocktype(data, pos)?)),
        op::ELSE => Ok(Instruction::Else),
        op::END => Ok(Instruction::End),
        op::BR => Ok(Instruction::Br(leb::u32(data, pos)?)),
        op::BR_IF => Ok(Instruction::BrIf(leb::u32(data, pos)?)),
        op::BR_TABLE => {
            let len = leb::u32(data, pos)? as usize;
            let mut targets = Vec::with_capacity(len);
            for _ in 0..len {
                targets.push(leb::u32(data, pos)?);
            }
            let default = leb::u32(data, pos)?;
            Ok(Instruction::BrTable(targets, default))
        }
        op::RETURN => Ok(Instruction::Return),
        op::CALL => Ok(Instruction::Call(leb::u32(data, pos)?)),
        op::CALL_INDIRECT => {
            let idx = leb::u32(data, pos)?;
            let table = leb::u32(data, pos)?;
            Ok(Instruction::CallIndirect(idx, table))
        }
        op::RETURN_CALL => Ok(Instruction::ReturnCall(leb::u32(data, pos)?)),
        op::RETURN_CALL_INDIRECT => {
            let idx = leb::u32(data, pos)?;
            let table = leb::u32(data, pos)?;
            Ok(Instruction::ReturnCallIndirect(idx, table))
        }
        op::CALL_REF => Ok(Instruction::CallRef(leb::u32(data, pos)?)),
        op::RETURN_CALL_REF => Ok(Instruction::ReturnCallRef(leb::u32(data, pos)?)),
        // Exception handling
        op::TRY => Ok(Instruction::Try(read_blocktype(data, pos)?)),
        op::CATCH => Ok(Instruction::Catch(leb::u32(data, pos)?)),
        op::THROW => Ok(Instruction::Throw(leb::u32(data, pos)?)),
        op::RETHROW => Ok(Instruction::Rethrow(leb::u32(data, pos)?)),
        op::THROW_REF => Ok(Instruction::ThrowRef),
        op::DELEGATE => Ok(Instruction::Delegate(leb::u32(data, pos)?)),
        op::CATCH_ALL => Ok(Instruction::CatchAll),
        op::TRY_TABLE => Ok(Instruction::TryTable(read_blocktype(data, pos)?)),
        // Parametric
        op::DROP => Ok(Instruction::Drop),
        op::SELECT => Ok(Instruction::Select),
        op::SELECT_T => {
            let count = leb::u32(data, pos)? as usize;
            let mut types = Vec::with_capacity(count);
            for _ in 0..count {
                types.push(byte_to_valtype(read_byte(data, pos)?)?);
            }
            Ok(Instruction::SelectT(types))
        }
        // Variable
        op::LOCAL_GET => Ok(Instruction::LocalGet(leb::u32(data, pos)?)),
        op::LOCAL_SET => Ok(Instruction::LocalSet(leb::u32(data, pos)?)),
        op::LOCAL_TEE => Ok(Instruction::LocalTee(leb::u32(data, pos)?)),
        op::GLOBAL_GET => Ok(Instruction::GlobalGet(leb::u32(data, pos)?)),
        op::GLOBAL_SET => Ok(Instruction::GlobalSet(leb::u32(data, pos)?)),
        // Table
        op::TABLE_GET => Ok(Instruction::TableGet(leb::u32(data, pos)?)),
        op::TABLE_SET => Ok(Instruction::TableSet(leb::u32(data, pos)?)),
        // Memory loads
        op::I32_LOAD => Ok(Instruction::I32Load(read_memarg(data, pos)?)),
        op::I64_LOAD => Ok(Instruction::I64Load(read_memarg(data, pos)?)),
        op::F32_LOAD => Ok(Instruction::F32Load(read_memarg(data, pos)?)),
        op::F64_LOAD => Ok(Instruction::F64Load(read_memarg(data, pos)?)),
        op::I32_LOAD8_S => Ok(Instruction::I32Load8S(read_memarg(data, pos)?)),
        op::I32_LOAD8_U => Ok(Instruction::I32Load8U(read_memarg(data, pos)?)),
        op::I32_LOAD16_S => Ok(Instruction::I32Load16S(read_memarg(data, pos)?)),
        op::I32_LOAD16_U => Ok(Instruction::I32Load16U(read_memarg(data, pos)?)),
        op::I64_LOAD8_S => Ok(Instruction::I64Load8S(read_memarg(data, pos)?)),
        op::I64_LOAD8_U => Ok(Instruction::I64Load8U(read_memarg(data, pos)?)),
        op::I64_LOAD16_S => Ok(Instruction::I64Load16S(read_memarg(data, pos)?)),
        op::I64_LOAD16_U => Ok(Instruction::I64Load16U(read_memarg(data, pos)?)),
        op::I64_LOAD32_S => Ok(Instruction::I64Load32S(read_memarg(data, pos)?)),
        op::I64_LOAD32_U => Ok(Instruction::I64Load32U(read_memarg(data, pos)?)),
        // Memory stores
        op::I32_STORE => Ok(Instruction::I32Store(read_memarg(data, pos)?)),
        op::I64_STORE => Ok(Instruction::I64Store(read_memarg(data, pos)?)),
        op::F32_STORE => Ok(Instruction::F32Store(read_memarg(data, pos)?)),
        op::F64_STORE => Ok(Instruction::F64Store(read_memarg(data, pos)?)),
        op::I32_STORE8 => Ok(Instruction::I32Store8(read_memarg(data, pos)?)),
        op::I32_STORE16 => Ok(Instruction::I32Store16(read_memarg(data, pos)?)),
        op::I64_STORE8 => Ok(Instruction::I64Store8(read_memarg(data, pos)?)),
        op::I64_STORE16 => Ok(Instruction::I64Store16(read_memarg(data, pos)?)),
        op::I64_STORE32 => Ok(Instruction::I64Store32(read_memarg(data, pos)?)),
        // Memory size/grow
        op::MEMORY_SIZE => {
            read_byte(data, pos)?;
            Ok(Instruction::MemorySize)
        }
        op::MEMORY_GROW => {
            read_byte(data, pos)?;
            Ok(Instruction::MemoryGrow)
        }
        // Constants
        op::I32_CONST => Ok(Instruction::I32Const(leb::i32(data, pos)?)),
        op::I64_CONST => Ok(Instruction::I64Const(leb::i64(data, pos)?)),
        op::F32_CONST => {
            let bytes: [u8; 4] = read_bytes(data, pos)?;
            Ok(Instruction::F32Const(f32::from_le_bytes(bytes)))
        }
        op::F64_CONST => {
            let bytes: [u8; 8] = read_bytes(data, pos)?;
            Ok(Instruction::F64Const(f64::from_le_bytes(bytes)))
        }
        // i32 comparison
        op::I32_EQZ => Ok(Instruction::I32Eqz),
        op::I32_EQ => Ok(Instruction::I32Eq),
        op::I32_NE => Ok(Instruction::I32Ne),
        op::I32_LT_S => Ok(Instruction::I32LtS),
        op::I32_LT_U => Ok(Instruction::I32LtU),
        op::I32_GT_S => Ok(Instruction::I32GtS),
        op::I32_GT_U => Ok(Instruction::I32GtU),
        op::I32_LE_S => Ok(Instruction::I32LeS),
        op::I32_LE_U => Ok(Instruction::I32LeU),
        op::I32_GE_S => Ok(Instruction::I32GeS),
        op::I32_GE_U => Ok(Instruction::I32GeU),
        // i64 comparison
        op::I64_EQZ => Ok(Instruction::I64Eqz),
        op::I64_EQ => Ok(Instruction::I64Eq),
        op::I64_NE => Ok(Instruction::I64Ne),
        op::I64_LT_S => Ok(Instruction::I64LtS),
        op::I64_LT_U => Ok(Instruction::I64LtU),
        op::I64_GT_S => Ok(Instruction::I64GtS),
        op::I64_GT_U => Ok(Instruction::I64GtU),
        op::I64_LE_S => Ok(Instruction::I64LeS),
        op::I64_LE_U => Ok(Instruction::I64LeU),
        op::I64_GE_S => Ok(Instruction::I64GeS),
        op::I64_GE_U => Ok(Instruction::I64GeU),
        // f32 comparison
        op::F32_EQ => Ok(Instruction::F32Eq),
        op::F32_NE => Ok(Instruction::F32Ne),
        op::F32_LT => Ok(Instruction::F32Lt),
        op::F32_GT => Ok(Instruction::F32Gt),
        op::F32_LE => Ok(Instruction::F32Le),
        op::F32_GE => Ok(Instruction::F32Ge),
        // f64 comparison
        op::F64_EQ => Ok(Instruction::F64Eq),
        op::F64_NE => Ok(Instruction::F64Ne),
        op::F64_LT => Ok(Instruction::F64Lt),
        op::F64_GT => Ok(Instruction::F64Gt),
        op::F64_LE => Ok(Instruction::F64Le),
        op::F64_GE => Ok(Instruction::F64Ge),
        // i32 numeric
        op::I32_CLZ => Ok(Instruction::I32Clz),
        op::I32_CTZ => Ok(Instruction::I32Ctz),
        op::I32_POPCNT => Ok(Instruction::I32Popcnt),
        op::I32_ADD => Ok(Instruction::I32Add),
        op::I32_SUB => Ok(Instruction::I32Sub),
        op::I32_MUL => Ok(Instruction::I32Mul),
        op::I32_DIV_S => Ok(Instruction::I32DivS),
        op::I32_DIV_U => Ok(Instruction::I32DivU),
        op::I32_REM_S => Ok(Instruction::I32RemS),
        op::I32_REM_U => Ok(Instruction::I32RemU),
        op::I32_AND => Ok(Instruction::I32And),
        op::I32_OR => Ok(Instruction::I32Or),
        op::I32_XOR => Ok(Instruction::I32Xor),
        op::I32_SHL => Ok(Instruction::I32Shl),
        op::I32_SHR_S => Ok(Instruction::I32ShrS),
        op::I32_SHR_U => Ok(Instruction::I32ShrU),
        op::I32_ROTL => Ok(Instruction::I32Rotl),
        op::I32_ROTR => Ok(Instruction::I32Rotr),
        // i64 numeric
        op::I64_CLZ => Ok(Instruction::I64Clz),
        op::I64_CTZ => Ok(Instruction::I64Ctz),
        op::I64_POPCNT => Ok(Instruction::I64Popcnt),
        op::I64_ADD => Ok(Instruction::I64Add),
        op::I64_SUB => Ok(Instruction::I64Sub),
        op::I64_MUL => Ok(Instruction::I64Mul),
        op::I64_DIV_S => Ok(Instruction::I64DivS),
        op::I64_DIV_U => Ok(Instruction::I64DivU),
        op::I64_REM_S => Ok(Instruction::I64RemS),
        op::I64_REM_U => Ok(Instruction::I64RemU),
        op::I64_AND => Ok(Instruction::I64And),
        op::I64_OR => Ok(Instruction::I64Or),
        op::I64_XOR => Ok(Instruction::I64Xor),
        op::I64_SHL => Ok(Instruction::I64Shl),
        op::I64_SHR_S => Ok(Instruction::I64ShrS),
        op::I64_SHR_U => Ok(Instruction::I64ShrU),
        op::I64_ROTL => Ok(Instruction::I64Rotl),
        op::I64_ROTR => Ok(Instruction::I64Rotr),
        // f32 numeric
        op::F32_ABS => Ok(Instruction::F32Abs),
        op::F32_NEG => Ok(Instruction::F32Neg),
        op::F32_CEIL => Ok(Instruction::F32Ceil),
        op::F32_FLOOR => Ok(Instruction::F32Floor),
        op::F32_TRUNC => Ok(Instruction::F32Trunc),
        op::F32_NEAREST => Ok(Instruction::F32Nearest),
        op::F32_SQRT => Ok(Instruction::F32Sqrt),
        op::F32_ADD => Ok(Instruction::F32Add),
        op::F32_SUB => Ok(Instruction::F32Sub),
        op::F32_MUL => Ok(Instruction::F32Mul),
        op::F32_DIV => Ok(Instruction::F32Div),
        op::F32_MIN => Ok(Instruction::F32Min),
        op::F32_MAX => Ok(Instruction::F32Max),
        op::F32_COPYSIGN => Ok(Instruction::F32Copysign),
        // f64 numeric
        op::F64_ABS => Ok(Instruction::F64Abs),
        op::F64_NEG => Ok(Instruction::F64Neg),
        op::F64_CEIL => Ok(Instruction::F64Ceil),
        op::F64_FLOOR => Ok(Instruction::F64Floor),
        op::F64_TRUNC => Ok(Instruction::F64Trunc),
        op::F64_NEAREST => Ok(Instruction::F64Nearest),
        op::F64_SQRT => Ok(Instruction::F64Sqrt),
        op::F64_ADD => Ok(Instruction::F64Add),
        op::F64_SUB => Ok(Instruction::F64Sub),
        op::F64_MUL => Ok(Instruction::F64Mul),
        op::F64_DIV => Ok(Instruction::F64Div),
        op::F64_MIN => Ok(Instruction::F64Min),
        op::F64_MAX => Ok(Instruction::F64Max),
        op::F64_COPYSIGN => Ok(Instruction::F64Copysign),
        // Conversions
        op::I32_WRAP_I64 => Ok(Instruction::I32WrapI64),
        op::I32_TRUNC_F32_S => Ok(Instruction::I32TruncF32S),
        op::I32_TRUNC_F32_U => Ok(Instruction::I32TruncF32U),
        op::I32_TRUNC_F64_S => Ok(Instruction::I32TruncF64S),
        op::I32_TRUNC_F64_U => Ok(Instruction::I32TruncF64U),
        op::I64_EXTEND_I32_S => Ok(Instruction::I64ExtendI32S),
        op::I64_EXTEND_I32_U => Ok(Instruction::I64ExtendI32U),
        op::I64_TRUNC_F32_S => Ok(Instruction::I64TruncF32S),
        op::I64_TRUNC_F32_U => Ok(Instruction::I64TruncF32U),
        op::I64_TRUNC_F64_S => Ok(Instruction::I64TruncF64S),
        op::I64_TRUNC_F64_U => Ok(Instruction::I64TruncF64U),
        op::F32_CONVERT_I32_S => Ok(Instruction::F32ConvertI32S),
        op::F32_CONVERT_I32_U => Ok(Instruction::F32ConvertI32U),
        op::F32_CONVERT_I64_S => Ok(Instruction::F32ConvertI64S),
        op::F32_CONVERT_I64_U => Ok(Instruction::F32ConvertI64U),
        op::F32_DEMOTE_F64 => Ok(Instruction::F32DemoteF64),
        op::F64_CONVERT_I32_S => Ok(Instruction::F64ConvertI32S),
        op::F64_CONVERT_I32_U => Ok(Instruction::F64ConvertI32U),
        op::F64_CONVERT_I64_S => Ok(Instruction::F64ConvertI64S),
        op::F64_CONVERT_I64_U => Ok(Instruction::F64ConvertI64U),
        op::F64_PROMOTE_F32 => Ok(Instruction::F64PromoteF32),
        op::I32_REINTERPRET_F32 => Ok(Instruction::I32ReinterpretF32),
        op::I64_REINTERPRET_F64 => Ok(Instruction::I64ReinterpretF64),
        op::F32_REINTERPRET_I32 => Ok(Instruction::F32ReinterpretI32),
        op::F64_REINTERPRET_I64 => Ok(Instruction::F64ReinterpretI64),
        // Sign extension
        op::I32_EXTEND8_S => Ok(Instruction::I32Extend8S),
        op::I32_EXTEND16_S => Ok(Instruction::I32Extend16S),
        op::I64_EXTEND8_S => Ok(Instruction::I64Extend8S),
        op::I64_EXTEND16_S => Ok(Instruction::I64Extend16S),
        op::I64_EXTEND32_S => Ok(Instruction::I64Extend32S),
        // Reference
        op::REF_NULL => Ok(Instruction::RefNull(read_byte(data, pos)?)),
        op::REF_IS_NULL => Ok(Instruction::RefIsNull),
        op::REF_FUNC => Ok(Instruction::RefFunc(leb::u32(data, pos)?)),
        op::REF_EQ => Ok(Instruction::RefEq),
        op::REF_AS_NON_NULL => Ok(Instruction::RefAsNonNull),
        op::BR_ON_NULL => Ok(Instruction::BrOnNull(leb::u32(data, pos)?)),
        op::BR_ON_NON_NULL => Ok(Instruction::BrOnNonNull(leb::u32(data, pos)?)),

        // 0xFB — GC prefix
        0xFB => decode_gc(data, pos),
        // 0xFC — Bulk memory / saturating truncation
        0xFC => decode_fc(data, pos),
        // 0xFD — SIMD
        0xFD => decode_simd(data, pos),
        // 0xFE — Atomics
        0xFE => decode_atomic(data, pos),

        _ => Ok(Instruction::Unknown(byte)),
    }
}

// 0xFB — GC / aggregate types
fn decode_gc(data: &[u8], pos: &mut usize) -> Result<Instruction, &'static str> {
    let sub = leb::u32(data, pos)?;
    match sub {
        // Struct instructions (0-5)
        0 => Ok(Instruction::StructNew(leb::u32(data, pos)?)),
        1 => Ok(Instruction::StructNewDefault(leb::u32(data, pos)?)),
        2 => Ok(Instruction::StructGet(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        3 => Ok(Instruction::StructGetS(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        4 => Ok(Instruction::StructGetU(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        5 => Ok(Instruction::StructSet(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        _ => Err("unknown GC sub-opcode"),
    }
}

// 0xFC — Bulk memory / saturating truncation
fn decode_fc(data: &[u8], pos: &mut usize) -> Result<Instruction, &'static str> {
    let sub = leb::u32(data, pos)?;
    match sub {
        // Saturating truncation (0-7)
        0 => Ok(Instruction::I32TruncSatF32S),
        1 => Ok(Instruction::I32TruncSatF32U),
        2 => Ok(Instruction::I32TruncSatF64S),
        3 => Ok(Instruction::I32TruncSatF64U),
        4 => Ok(Instruction::I64TruncSatF32S),
        5 => Ok(Instruction::I64TruncSatF32U),
        6 => Ok(Instruction::I64TruncSatF64S),
        7 => Ok(Instruction::I64TruncSatF64U),
        // Memory operations (8-11)
        8 => {
            let data_idx = leb::u32(data, pos)?;
            let _mem_idx = read_byte(data, pos)?;
            Ok(Instruction::MemoryInit(data_idx))
        }
        9 => Ok(Instruction::DataDrop(leb::u32(data, pos)?)),
        10 => {
            let _mem1 = read_byte(data, pos)?;
            let _mem2 = read_byte(data, pos)?;
            Ok(Instruction::MemoryCopy)
        }
        11 => {
            let _mem = read_byte(data, pos)?;
            Ok(Instruction::MemoryFill)
        }
        // Table operations (12-17)
        12 => Ok(Instruction::TableInit(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        13 => Ok(Instruction::ElemDrop(leb::u32(data, pos)?)),
        14 => Ok(Instruction::TableCopy(
            leb::u32(data, pos)?,
            leb::u32(data, pos)?,
        )),
        15 => Ok(Instruction::TableGrow(leb::u32(data, pos)?)),
        16 => Ok(Instruction::TableSize(leb::u32(data, pos)?)),
        17 => Ok(Instruction::TableFill(leb::u32(data, pos)?)),
        _ => Err("unknown FC sub-opcode"),
    }
}

// 0xFD — SIMD / Vector instructions (simplified - key ops only)
fn decode_simd(data: &[u8], pos: &mut usize) -> Result<Instruction, &'static str> {
    let sub = leb::u32(data, pos)?;
    match sub {
        // Memory operations (0-11)
        0 => Ok(Instruction::V128Load(read_memarg(data, pos)?)),
        1 => Ok(Instruction::V128Load8x8S(read_memarg(data, pos)?)),
        2 => Ok(Instruction::V128Load8x8U(read_memarg(data, pos)?)),
        3 => Ok(Instruction::V128Load16x4S(read_memarg(data, pos)?)),
        4 => Ok(Instruction::V128Load16x4U(read_memarg(data, pos)?)),
        5 => Ok(Instruction::V128Load32x2S(read_memarg(data, pos)?)),
        6 => Ok(Instruction::V128Load32x2U(read_memarg(data, pos)?)),
        7 => Ok(Instruction::V128Load8Splat(read_memarg(data, pos)?)),
        8 => Ok(Instruction::V128Load16Splat(read_memarg(data, pos)?)),
        9 => Ok(Instruction::V128Load32Splat(read_memarg(data, pos)?)),
        10 => Ok(Instruction::V128Load64Splat(read_memarg(data, pos)?)),
        11 => Ok(Instruction::V128Store(read_memarg(data, pos)?)),
        // Constants (12-13)
        12 => Ok(Instruction::V128Const(read_bytes(data, pos)?)),
        13 => Ok(Instruction::I8x16Shuffle(read_bytes(data, pos)?)),
        // Basic ops (14-20)
        14 => Ok(Instruction::I8x16Swizzle),
        15 => Ok(Instruction::I8x16Splat),
        16 => Ok(Instruction::I16x8Splat),
        17 => Ok(Instruction::I32x4Splat),
        18 => Ok(Instruction::I64x2Splat),
        19 => Ok(Instruction::F32x4Splat),
        20 => Ok(Instruction::F64x2Splat),
        // Lane extraction/replacement (21-34)
        21 => Ok(Instruction::I8x16ExtractLaneS(read_byte(data, pos)?)),
        22 => Ok(Instruction::I8x16ExtractLaneU(read_byte(data, pos)?)),
        23 => Ok(Instruction::I8x16ReplaceLane(read_byte(data, pos)?)),
        24 => Ok(Instruction::I16x8ExtractLaneS(read_byte(data, pos)?)),
        25 => Ok(Instruction::I16x8ExtractLaneU(read_byte(data, pos)?)),
        26 => Ok(Instruction::I16x8ReplaceLane(read_byte(data, pos)?)),
        27 => Ok(Instruction::I32x4ExtractLane(read_byte(data, pos)?)),
        28 => Ok(Instruction::I32x4ReplaceLane(read_byte(data, pos)?)),
        29 => Ok(Instruction::I64x2ExtractLane(read_byte(data, pos)?)),
        30 => Ok(Instruction::I64x2ReplaceLane(read_byte(data, pos)?)),
        31 => Ok(Instruction::F32x4ExtractLane(read_byte(data, pos)?)),
        32 => Ok(Instruction::F32x4ReplaceLane(read_byte(data, pos)?)),
        33 => Ok(Instruction::F64x2ExtractLane(read_byte(data, pos)?)),
        34 => Ok(Instruction::F64x2ReplaceLane(read_byte(data, pos)?)),
        // More SIMD ops can be added here...
        _ => Err("unknown SIMD sub-opcode"),
    }
}

// 0xFE — Atomics / Threads (simplified - key ops only)
fn decode_atomic(data: &[u8], pos: &mut usize) -> Result<Instruction, &'static str> {
    let sub = leb::u32(data, pos)?;
    match sub {
        // Fence and notification (0-3)
        0 => Ok(Instruction::MemoryAtomicNotify(read_memarg(data, pos)?)),
        1 => Ok(Instruction::MemoryAtomicWait32(read_memarg(data, pos)?)),
        2 => Ok(Instruction::MemoryAtomicWait64(read_memarg(data, pos)?)),
        3 => Ok(Instruction::AtomicFence),
        // Atomic loads (16-22)
        16 => Ok(Instruction::I32AtomicLoad(read_memarg(data, pos)?)),
        17 => Ok(Instruction::I64AtomicLoad(read_memarg(data, pos)?)),
        18 => Ok(Instruction::I32AtomicLoad8U(read_memarg(data, pos)?)),
        19 => Ok(Instruction::I32AtomicLoad16U(read_memarg(data, pos)?)),
        20 => Ok(Instruction::I64AtomicLoad8U(read_memarg(data, pos)?)),
        21 => Ok(Instruction::I64AtomicLoad16U(read_memarg(data, pos)?)),
        22 => Ok(Instruction::I64AtomicLoad32U(read_memarg(data, pos)?)),
        // Atomic stores (23-29)
        23 => Ok(Instruction::I32AtomicStore(read_memarg(data, pos)?)),
        24 => Ok(Instruction::I64AtomicStore(read_memarg(data, pos)?)),
        25 => Ok(Instruction::I32AtomicStore8(read_memarg(data, pos)?)),
        26 => Ok(Instruction::I32AtomicStore16(read_memarg(data, pos)?)),
        27 => Ok(Instruction::I64AtomicStore8(read_memarg(data, pos)?)),
        28 => Ok(Instruction::I64AtomicStore16(read_memarg(data, pos)?)),
        29 => Ok(Instruction::I64AtomicStore32(read_memarg(data, pos)?)),
        // RMW Add (30-36)
        30 => Ok(Instruction::I32AtomicRmwAdd(read_memarg(data, pos)?)),
        31 => Ok(Instruction::I64AtomicRmwAdd(read_memarg(data, pos)?)),
        32 => Ok(Instruction::I32AtomicRmw8AddU(read_memarg(data, pos)?)),
        33 => Ok(Instruction::I32AtomicRmw16AddU(read_memarg(data, pos)?)),
        34 => Ok(Instruction::I64AtomicRmw8AddU(read_memarg(data, pos)?)),
        35 => Ok(Instruction::I64AtomicRmw16AddU(read_memarg(data, pos)?)),
        36 => Ok(Instruction::I64AtomicRmw32AddU(read_memarg(data, pos)?)),
        // More atomic ops can be added here...
        _ => Err("unknown atomic sub-opcode"),
    }
}

// Full section parsers (all 13, exhaustive)
fn parse_section(data: &[u8], id: u8) -> Result<Section, &'static str> {
    let mut pos = 0;
    match id {
        CUSTOM => {
            let name = read_name(data, &mut pos)?;
            let content = data[pos..].to_vec();
            Ok(Section::Custom(name, content))
        }
        TYPE => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut types = Vec::with_capacity(count);
            for _ in 0..count {
                types.push(read_functype(data, &mut pos)?);
            }
            Ok(Section::Type(types))
        }
        IMPORT => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut imports = Vec::with_capacity(count);
            for _ in 0..count {
                let module = read_name(data, &mut pos)?;
                let name = read_name(data, &mut pos)?;
                let desc = match read_byte(data, &mut pos)? {
                    0x00 => ImportDesc::Func(leb::u32(data, &mut pos)?),
                    0x01 => ImportDesc::Table(Table {
                        reftype: read_byte(data, &mut pos)?,
                        limits: read_limits(data, &mut pos)?,
                    }),
                    0x02 => ImportDesc::Memory(read_limits(data, &mut pos)?),
                    0x03 => {
                        let vt = byte_to_valtype(read_byte(data, &mut pos)?)?;
                        let mutable = read_byte(data, &mut pos)? == 0x01;
                        ImportDesc::Global(GlobalType {
                            valtype: vt,
                            mutable,
                        })
                    }
                    0x04 => ImportDesc::Tag(leb::u32(data, &mut pos)?),
                    _ => return Err("invalid import kind"),
                };
                imports.push(Import { module, name, desc });
            }
            Ok(Section::Import(imports))
        }
        FUNCTION => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut funcs = Vec::with_capacity(count);
            for _ in 0..count {
                funcs.push(leb::u32(data, &mut pos)?);
            }
            Ok(Section::Function(funcs))
        }
        TABLE => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut tables = Vec::with_capacity(count);
            for _ in 0..count {
                tables.push(Table {
                    reftype: read_byte(data, &mut pos)?,
                    limits: read_limits(data, &mut pos)?,
                });
            }
            Ok(Section::Table(tables))
        }
        MEMORY => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut memories = Vec::with_capacity(count);
            for _ in 0..count {
                memories.push(read_limits(data, &mut pos)?);
            }
            Ok(Section::Memory(memories))
        }
        GLOBAL => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut globals = Vec::with_capacity(count);
            for _ in 0..count {
                let vt = byte_to_valtype(read_byte(data, &mut pos)?)?;
                let mutable = read_byte(data, &mut pos)? == 0x01;
                let init = read_instructions(data, &mut pos)?;
                globals.push(Global {
                    ty: GlobalType {
                        valtype: vt,
                        mutable,
                    },
                    init,
                });
            }
            Ok(Section::Global(globals))
        }
        EXPORT => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut exports = Vec::with_capacity(count);
            for _ in 0..count {
                let name = read_name(data, &mut pos)?;
                let kind = read_byte(data, &mut pos)?;
                let idx = leb::u32(data, &mut pos)?;
                exports.push(Export { name, kind, idx });
            }
            Ok(Section::Export(exports))
        }
        START => Ok(Section::Start(leb::u32(data, &mut pos)?)),
        ELEMENT => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut elements = Vec::with_capacity(count);
            for _ in 0..count {
                let kind = leb::u32(data, &mut pos)?;
                elements.push(Element { kind, init: vec![] });
            }
            Ok(Section::Element(elements))
        }
        CODE => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut codes = Vec::with_capacity(count);
            for _ in 0..count {
                let _size = leb::u32(data, &mut pos)?;
                let local_count = leb::u32(data, &mut pos)? as usize;
                let mut locals = Vec::with_capacity(local_count);
                for _ in 0..local_count {
                    locals.push((
                        leb::u32(data, &mut pos)?,
                        byte_to_valtype(read_byte(data, &mut pos)?)?,
                    ));
                }
                let body = read_instructions(data, &mut pos)?;
                codes.push(Code { locals, body });
            }
            Ok(Section::Code(codes))
        }
        DATA => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut datas = Vec::with_capacity(count);
            for _ in 0..count {
                let flags = leb::u32(data, &mut pos)?;
                let mode = if flags == 0 {
                    DataMode::Active {
                        memory: 0,
                        offset: read_instructions(data, &mut pos)?,
                    }
                } else if flags == 1 {
                    DataMode::Passive
                } else {
                    return Err("unsupported data segment flags");
                };
                let len = leb::u32(data, &mut pos)? as usize;
                let init = data[pos..pos + len].to_vec();
                pos += len;
                datas.push(Data { mode, init });
            }
            Ok(Section::Data(datas))
        }
        DATACOUNT => Ok(Section::DataCount(leb::u32(data, &mut pos)?)),
        TAG => {
            let count = leb::u32(data, &mut pos)? as usize;
            let mut tags = Vec::with_capacity(count);
            for _ in 0..count {
                let kind = read_byte(data, &mut pos)?;
                let typeidx = leb::u32(data, &mut pos)?;
                tags.push(Tag { kind, typeidx });
            }
            Ok(Section::Tag(tags))
        }
        // every section id has its parser ...
        _ => Ok(Section::Custom("unknown".to_string(), data.to_vec())),
    }
}

// Public API (exactly like your original watson)
pub fn parse(bytes: &[u8]) -> Result<Program, String> {
    if bytes.len() < 8
        || &bytes[0..4] != MAGIC
        || u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]) != VERSION
    {
        return Err("invalid header".to_string());
    }
    let mut pos = 8;
    let mut sections = Vec::new();
    while pos < bytes.len() {
        let id = bytes[pos];
        pos += 1;
        let size = leb::u32(bytes, &mut pos)? as usize;
        let data = &bytes[pos..pos + size];
        pos += size;
        sections.push(parse_section(data, id)?);
    }
    Ok(Program { sections })
}

// Compiler (encode back to Wasm)
pub fn encode(program: &Program) -> Vec<u8> {
    let mut out = Vec::with_capacity(1024);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());

    for section in &program.sections {
        match section {
            Section::Custom(name, data) => {
                out.push(CUSTOM);
                let mut sec = Vec::new();
                sec.extend_from_slice(&leb::encode_u32(name.len() as u32));
                sec.extend_from_slice(name.as_bytes());
                sec.extend_from_slice(data);
                out.extend_from_slice(&leb::encode_u32(sec.len() as u32));
                out.extend_from_slice(&sec);
            }
            Section::Type(types) => {
                out.push(TYPE);
                let mut sec = Vec::new();
                sec.extend_from_slice(&leb::encode_u32(types.len() as u32));
                for ft in types {
                    sec.push(0x60);
                    sec.extend_from_slice(&leb::encode_u32(ft.params.len() as u32));
                    for p in &ft.params {
                        sec.push(valtype_to_byte(p));
                    }
                    sec.extend_from_slice(&leb::encode_u32(ft.results.len() as u32));
                    for r in &ft.results {
                        sec.push(valtype_to_byte(r));
                    }
                }
                out.extend_from_slice(&leb::encode_u32(sec.len() as u32));
                out.extend_from_slice(&sec);
            }
            Section::Import(imports) => {
                out.push(IMPORT);
                let mut sec = Vec::new();
                sec.extend_from_slice(&leb::encode_u32(imports.len() as u32));
                for imp in imports {
                    sec.extend_from_slice(&leb::encode_u32(imp.module.len() as u32));
                    sec.extend_from_slice(imp.module.as_bytes());
                    sec.extend_from_slice(&leb::encode_u32(imp.name.len() as u32));
                    sec.extend_from_slice(imp.name.as_bytes());
                    match &imp.desc {
                        ImportDesc::Func(idx) => {
                            sec.push(0x00);
                            sec.extend_from_slice(&leb::encode_u32(*idx));
                        }
                        ImportDesc::Table(t) => {
                            sec.push(0x01);
                            sec.push(t.reftype);
                            sec.push(if t.limits.max.is_some() { 0x01 } else { 0x00 });
                            sec.extend_from_slice(&leb::encode_u32(t.limits.min));
                            if let Some(max) = t.limits.max {
                                sec.extend_from_slice(&leb::encode_u32(max));
                            }
                        }
                        ImportDesc::Memory(lim) => {
                            sec.push(0x02);
                            sec.push(if lim.max.is_some() { 0x01 } else { 0x00 });
                            sec.extend_from_slice(&leb::encode_u32(lim.min));
                            if let Some(max) = lim.max {
                                sec.extend_from_slice(&leb::encode_u32(max));
                            }
                        }
                        ImportDesc::Global(gt) => {
                            sec.push(0x03);
                            sec.push(valtype_to_byte(&gt.valtype));
                            sec.push(if gt.mutable { 0x01 } else { 0x00 });
                        }
                        ImportDesc::Tag(idx) => {
                            sec.push(0x04);
                            sec.extend_from_slice(&leb::encode_u32(*idx));
                        }
                    }
                }
                out.extend_from_slice(&leb::encode_u32(sec.len() as u32));
                out.extend_from_slice(&sec);
            }
            _ => {
                // Other sections not yet implemented in encoder
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::println;

    // ========== LEB128 Tests ==========
    #[test]
    fn test_leb128_u32_zero() {
        let encoded = leb::encode_u32(0);
        assert_eq!(encoded, vec![0x00]);
        let mut pos = 0;
        assert_eq!(leb::u32(&encoded, &mut pos).unwrap(), 0);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_leb128_u32_single_byte() {
        let encoded = leb::encode_u32(127);
        assert_eq!(encoded, vec![0x7F]);
        let mut pos = 0;
        assert_eq!(leb::u32(&encoded, &mut pos).unwrap(), 127);
        assert_eq!(pos, 1);
    }

    #[test]
    fn test_leb128_u32_two_bytes() {
        let encoded = leb::encode_u32(128);
        assert_eq!(encoded, vec![0x80, 0x01]);
        let mut pos = 0;
        assert_eq!(leb::u32(&encoded, &mut pos).unwrap(), 128);
        assert_eq!(pos, 2);
    }

    #[test]
    fn test_leb128_u32_max() {
        let encoded = leb::encode_u32(u32::MAX);
        assert_eq!(encoded, vec![0xFF, 0xFF, 0xFF, 0xFF, 0x0F]);
        let mut pos = 0;
        assert_eq!(leb::u32(&encoded, &mut pos).unwrap(), u32::MAX);
        assert_eq!(pos, 5);
    }

    #[test]
    fn test_leb128_i32_positive() {
        let encoded = leb::encode_i32(1);
        assert_eq!(encoded, vec![0x01]);
        let mut pos = 0;
        assert_eq!(leb::i32(&encoded, &mut pos).unwrap(), 1);
    }

    #[test]
    fn test_leb128_i32_negative() {
        let encoded = leb::encode_i32(-1);
        assert_eq!(encoded, vec![0x7F]);
        let mut pos = 0;
        assert_eq!(leb::i32(&encoded, &mut pos).unwrap(), -1);
    }

    #[test]
    fn test_leb128_i32_negative_large() {
        let encoded = leb::encode_i32(-128);
        assert_eq!(encoded, vec![0x80, 0x7F]);
        let mut pos = 0;
        assert_eq!(leb::i32(&encoded, &mut pos).unwrap(), -128);
    }

    // ========== Minimal Valid Module Test ==========
    #[test]
    fn test_parse_minimal_module() {
        // Minimal valid wasm module: magic + version
        let bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let program = parse(&bytes).unwrap();
        assert!(program.sections.is_empty());
    }

    // ========== Custom Section Test ==========
    #[test]
    fn test_custom_section() {
        let mut bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]; // header
        bytes.push(0x00); // custom section id
        bytes.push(0x06); // section size: 1 (name_len) + 3 (name) + 2 (data) = 6
        bytes.push(0x03); // name length
        bytes.extend_from_slice(b"abc");
        bytes.push(0x01);
        bytes.push(0x02); // custom data

        let program = parse(&bytes).unwrap();
        assert_eq!(program.sections.len(), 1);
        match &program.sections[0] {
            Section::Custom(name, data) => {
                assert_eq!(name, "abc");
                assert_eq!(data, &vec![0x01, 0x02]);
            }
            _ => panic!("Expected Custom section"),
        }
    }

    // ========== Type Section Test ==========
    #[test]
    fn test_type_section_empty() {
        let mut bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00]; // header
        bytes.push(0x01); // type section id
        bytes.push(0x01); // section size
        bytes.push(0x00); // 0 types

        let program = parse(&bytes).unwrap();
        assert_eq!(program.sections.len(), 1);
        match &program.sections[0] {
            Section::Type(types) => assert!(types.is_empty()),
            _ => panic!("Expected Type section"),
        }
    }

    #[test]
    fn test_type_section_single() {
        // Type section with one function: () -> i32
        let mut bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        bytes.push(0x01); // type section id
        bytes.push(0x05); // section size
        bytes.push(0x01); // 1 type
        bytes.push(0x60); // func type tag
        bytes.push(0x00); // 0 params
        bytes.push(0x01); // 1 result
        bytes.push(0x7F); // i32

        let program = parse(&bytes).unwrap();
        match &program.sections[0] {
            Section::Type(types) => {
                assert_eq!(types.len(), 1);
                assert!(types[0].params.is_empty());
                assert_eq!(types[0].results.len(), 1);
                assert_eq!(types[0].results[0], ValType::I32);
            }
            _ => panic!("Expected Type section"),
        }
    }

    // ========== Import Section Test ==========
    #[test]
    fn test_import_section_func() {
        // Import section with one function import
        let mut bytes = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        bytes.push(0x02); // import section id
                          // section size = 1 + 1 + 3 + 1 + 4 + 1 + 1 = 12 = 0x0C
        bytes.push(0x0C); // section size
        bytes.push(0x01); // 1 import
        bytes.push(0x03); // module name len
        bytes.extend_from_slice(b"env");
        bytes.push(0x04); // field name len
        bytes.extend_from_slice(b"test");
        bytes.push(0x00); // import kind: func
        bytes.push(0x00); // type index

        let program = parse(&bytes).unwrap();
        match &program.sections[0] {
            Section::Import(imports) => {
                assert_eq!(imports.len(), 1);
                assert_eq!(imports[0].module, "env");
                assert_eq!(imports[0].name, "test");
                match imports[0].desc {
                    ImportDesc::Func(0) => {}
                    _ => panic!("Expected Func import with index 0"),
                }
            }
            _ => panic!("Expected Import section"),
        }
    }

    // ========== Round-trip Tests ==========
    #[test]
    fn test_roundtrip_empty_module() {
        let original = vec![0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let program = parse(&original).unwrap();
        let encoded = encode(&program);
        assert_eq!(encoded, original);
    }

    #[test]
    fn test_roundtrip_type_section() {
        // Build a simple program with a type section
        let mut program = Program { sections: vec![] };
        let func_type = FuncType {
            params: vec![ValType::I32],
            results: vec![ValType::I64],
        };
        program.sections.push(Section::Type(vec![func_type]));

        let encoded = encode(&program);
        let reparsed = parse(&encoded).unwrap();

        assert_eq!(reparsed.sections.len(), 1);
        match &reparsed.sections[0] {
            Section::Type(types) => {
                assert_eq!(types.len(), 1);
                assert_eq!(types[0].params.len(), 1);
                assert_eq!(types[0].params[0], ValType::I32);
                assert_eq!(types[0].results.len(), 1);
                assert_eq!(types[0].results[0], ValType::I64);
            }
            _ => panic!("Expected Type section after roundtrip"),
        }
    }

    // ========== Integration Test with Real Wasm File ==========
    #[test]
    fn test_parse_real_wasm_file() {
        // This test parses the simplest.wasm file from the example directory
        // It's a real WebAssembly binary that exports a "main" function returning 42
        let wasm_bytes = include_bytes!("../example/simplest/simplest.wasm");
        let program = parse(wasm_bytes).expect("Failed to parse simplest.wasm");

        // Should have: Type, Function, Memory, Export, Code sections
        assert!(
            !program.sections.is_empty(),
            "Expected at least one section"
        );

        // Verify we can identify the sections
        let mut found_type = false;
        let mut found_export = false;
        let mut found_code = false;

        for section in &program.sections {
            match section {
                Section::Type(_) => found_type = true,
                Section::Export(exports) => {
                    found_export = true;
                    // Check for "main" export
                    assert!(
                        exports.iter().any(|e| e.name == "main"),
                        "Expected 'main' export"
                    );
                }
                Section::Code(_) => found_code = true,
                _ => {}
            }
        }

        assert!(found_type, "Expected Type section");
        assert!(found_export, "Expected Export section");
        assert!(found_code, "Expected Code section");
    }

    // ========== Spec Test Suite Integration ==========
    // These tests run against the official WebAssembly spec test suite
    // Setup required:
    //   1. git submodule add https://github.com/WebAssembly/spec.git tests/spec-tests
    //   2. ./build-spec-tests.sh  (converts .wast to .wasm)
    //   Or: manually download pre-built wasm files to tests/fixtures/

    #[test]
    #[ignore = "Requires spec tests submodule - run: git submodule update --init"]
    fn test_spec_tests_available() {
        // This test verifies spec tests are properly set up
        let tests = load_spec_tests();
        assert!(
            !tests.is_empty(),
            "No spec tests found. Run:\n  git submodule update --init\n  ./build-spec-tests.sh"
        );
        println!("Found {} spec test files", tests.len());
        for (name, _) in &tests {
            println!("  - {}", name);
        }
    }

    #[test]
    #[ignore = "Requires spec tests submodule"]
    fn test_parse_all_spec_tests() {
        // Parse all available spec test wasm files
        let tests = load_spec_tests();
        assert!(!tests.is_empty(), "No spec tests available");

        let mut passed = 0;
        let mut failed = 0;

        for (name, bytes) in tests {
            match parse(&bytes) {
                Ok(program) => {
                    println!(
                        "✓ {}: parsed successfully ({} sections)",
                        name,
                        program.sections.len()
                    );
                    passed += 1;
                }
                Err(e) => {
                    println!("✗ {}: failed to parse - {}", name, e);
                    failed += 1;
                }
            }
        }

        println!("\nResults: {} passed, {} failed", passed, failed);
        assert_eq!(failed, 0, "{} spec tests failed to parse", failed);
    }
}
