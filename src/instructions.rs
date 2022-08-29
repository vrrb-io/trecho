#![allow(unused, unused_mut, dead_code)]
use crate::extensions::{Ext, I64, I32};
use std::collections::HashMap;
use crate::register::Register;
use crate::encoding::{OpCodeType, EncodingTable};
use crate::encoding_types::{OpCode, Inst};

// Enum with all instruction variants. Need all instructions for all extensions
// allow encoding table, based on extension to determine whether or not the
// instruction is ever used. i.e. Some I64 instructions are not included in
// I32 instructions, however, we do not want a separate Instruction enum for each
// When we implement decoding, based on the Extension set of the type of machine
// We will know which OpCodes are Invalid, because they will return an Invalid
// variant of the OpCodeType.
#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    // Invalid instruction, undefined
    Undefined,
    // Load upper immediate
    Lui { rd: Register, imm: i32 },
    // add upper immediate to program counter
    Auipc { rd: Register, imm: i32 },
    // jump and link
    Jal { rd: Register, imm: i32 },
    // jump and link register
    Jalr { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // equal
    Beq { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // not equal
    Bne { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // less than
    Blt { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // greater or equal
    Bge { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // less than unsigned
    Bltu { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // greater than equal unsigned
    Bgeu { rd: Register, rs1: Register, rs2: Register, imm: i32 },
    // load bit
    Lb { rd: Register, rs1: Register, imm: i32 },
    // load halfword
    Lh { rd: Register, rs1: Register, imm: i32 },
    // load bit unsigned
    Lbu { rd: Register, rs1: Register, imm: i32 },
    // load halfword unsigned
    Lhu { rd: Register, rs1: Register, imm: i32 },
    // save bit
    Sb { rs1: Register, imm: i32 },
    // save halfword
    Sh { rs1: Register, rs2: Register, imm: i32 },
    // save word
    Sw { rs1: Register, rs2: Register, imm: i32 },
    // add integer
    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
    Slli { rd: Register, rs1: Register, imm: i32 },
    Srli {},
    Srai {},
    Add {},
    Sub {},
    Sll {},
    Slt {},
    Sltu {},
    Xor {},
    Srl {},
    Sra {},
    Or {},
    And {},
    Fence {},
    Ecall,
    EBreak,
    Lwu {},
    Ld {},
    Sd {},
    Addiw {},
    Slliw {},
    Srliw {},
    Sraiw {},
    Addw {},
    Subw {},
    Sllw {},
    Srlw {},
    Sraw {},
    FenceI {},
    Csrrw {},
    Csrrs {},
    Csrrc {},
    Csrrwi {},
    Csrrsi {},
    Csrrci {},
    Mul {},
    Mulh {},
    Mulhsu {},
    Div {},
    Divu {},
    Rem {},
    Remu {},
    Mulw {},
    Divw {},
    Divuw {},
    Remw {},
    RemuW {},
    LrW {},
    ScW {},
    AmoswapW {},
    AmoaddW {},
    AmoxorW {},
    AmoandW {},
    AmoorW {},
    AmominW {},
    Amomax {},
    AmominuW {},
    AmomaxuW {},
    LrD {},
    ScD {},
    AmoswapD {},
    AmoaddD {},
    AmoxorD {},
    AmoandD {},
    AmoorD {},
    AmominD {},
    AmomaxD {},
    AmominuD {},
    AmomaxuD {},
    Flw {},
    Fsw {},
    FmaddS {},
    FmsubS {},
    FnmsubS {},
    FnmaddS {},
    FaddS {},
    FsubS {},
    FmulS {},
    FdivS {},
    FsqrtS {},
    FsgnjS {},
    FsgnjnS {},
    FsgnjxS {},
    FminS {},
    FmaxS {},
    FcvtWS {},
    FctvWUS {},
    FmvXW {},
    FeqS {},
    FltS {},
    FleS {},
    FclassS {},
    FcvtSW {},
    FcvtSWU {},
    FmvWX {},
    FcvtLS {},
    FcvtLUS {},
    FcvtSL {},
    FcvtSLU {},
    Fld {},
    Fsd {},
    FmaddD {},
    FmsubD {},
    FnmsubD {},
    FnmaddD {},
    FaddD {},
    FsubD {},
    FdivD {},
    FsqrtD {},
    FsgnjD {},
    FsgnjnD {},
    FsgnjxD {},
    FminD {},
    FmaxD {},
    FcvtSD {},
    FcvtDS {},
    FeqD {},
    FltD {},
    FleD {},
    FclassD {},
    FcvtWD {},
    FcvtWUD {},
    FcvtDW {},
    FcvtDWU {},
    FcvtLD {},
    FcvtLUD {},
    FmvXD {},
    FcvtDL {},
    FcvtDLU {},
    FmvDX {},
    Flq {},
    Fsq {},
    FmaddQ {},
    FmsubQ {},
    FnmsubQ {},
    FnmaddQ {},
    FaddQ {},
    FsubQ {},
    FmulQ {},
    FdivQ {},
    FsqrtQ {},
    FsgnjQ {},
    FsgnjnQ {},
    FsgnjxQ {},
    FminQ {},
    FmaxQ {},
    FcvtSQ {},
    FcvtQS {},
    FcvtDQ {},
    FcvtQD {},
    FeqQ {},
    FltQ {},
    FleQ {},
    FclassQ {},
    FcvtWQ {},
    FcvtWUQ {},
    FcvtQW {},
    FcvtQWU {},
    FcvtLQ {},
    FcvtLUQ {},
    FcvtQL {},
    FcvtQLU {},


    //TODO: Add additional instruction sets for other extensions not yet implemented
}
