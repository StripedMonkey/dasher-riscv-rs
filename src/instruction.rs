use std::fmt;

// This macro is zero indexed
#[macro_export]
macro_rules! extract_bits {
    ($data:ident[$offset:expr]) => { // type[start]
        (($data  >> $offset) & 1)
    };
    ($data:ident[$start:expr; $end:expr]) => {// type[start;end]
        (((1 << ($start - $end + 1)) - 1) & ($data >> $end))
    };
    (@extend $data:ident[$start:expr; $end:expr]) => { // @extend type[start;end]
        sign_extend(
            (((1 << ($start - $end + 1)) - 1) & ($data >> $end)),
            $start - $end + 1,
        )
    };
    (@extend $data:ident[$bit:expr]) => { // @extend type[bit]
        sign_extend(extract_bits!($data[$bit;$bit]), 1)
    };

    // TODO: Actually verify these are correct
    ($data:ident[$start:expr;]) => { // type[start;]
        ($data & ((1<<n)-1)),
    };
    ($data:ident[;$end:expr]) => { // type[;end]
        ($data >>> $end)
    };
}

pub(crate) fn sign_extend(instruction: i32, offset: usize) -> i32 {
    // shift left all but the last n bits
    let shifted = instruction << (32 - offset);
    // as a signed integer shift back
    shifted >> (32 - offset)
}

pub(crate) enum Instruction {
    LUI {
        imm: i32,
        rd: usize,
    },
    AUIPC {
        imm: i32,
        rd: usize,
    },
    JAL {
        imm: i32,
        rd: usize,
    },
    JALR {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    BEQ {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    BNE {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    BLT {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    BGE {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    BLTU {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    BGEU {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    LB {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    LH {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    LW {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    LBU {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    LHU {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    SB {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    SH {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    SW {
        imm: i32,
        rs2: usize,
        rs1: usize,
    },
    ADDI {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    SLTI {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    SLTIU {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    XORI {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    ORI {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    ANDI {
        imm: i32,
        rs1: usize,
        rd: usize,
    },
    SLLI {
        shamt: i32,
        rs1: usize,
        rd: usize,
    },
    SRLI {
        shamt: i32,
        rs1: usize,
        rd: usize,
    },
    SRAI {
        shamt: i32,
        rs1: usize,
        rd: usize,
    },
    ADD {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SUB {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SLL {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SLT {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SLTU {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    XOR {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SRL {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    SRA {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    OR {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    AND {
        rs2: usize,
        rs1: usize,
        rd: usize,
    },
    FENCE {
        fm: i32,
        pred: i32,
        succ: i32,
        rs1: usize,
        rd: usize,
    },
    ECALL,
    EBREAK,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::LUI { imm, rd } => write!(f, "LUI x{rd} #{imm:#x}"),
            Instruction::AUIPC { imm, rd } => write!(f, "AUIPC x{rd} #{imm:#x}"),
            Instruction::JAL { imm, rd } => write!(f, "JAL x{rd} #{imm:#x}"),
            Instruction::JALR { imm, rs1, rd } => write!(f, "JALR x{rd} x{rs1} #{imm:#x}"),
            Instruction::BEQ { imm, rs2, rs1 } => write!(f, "BEQ x{rs1} x{rs2} #{imm:#x}"),
            Instruction::BNE { imm, rs2, rs1 } => write!(f, "BNE x{rs1} x{rs2} #{imm:#x}"),
            Instruction::BLT { imm, rs2, rs1 } => write!(f, "BLT x{rs1} x{rs2} #{imm:#x}"),
            Instruction::BGE { imm, rs2, rs1 } => write!(f, "BGE x{rs1} x{rs2} #{imm:#x}"),
            Instruction::BLTU { imm, rs2, rs1 } => write!(f, "BLTU x{rs1} x{rs2} #{imm:#x}"),
            Instruction::BGEU { imm, rs2, rs1 } => write!(f, "BGEU x{rs1} x{rs2} #{imm:#x}"),
            Instruction::LB { imm, rs1, rd } => write!(f, "LB x{rd} x{rs1} #{imm:#x}"),
            Instruction::LH { imm, rs1, rd } => write!(f, "LH x{rd} x{rs1} #{imm:#x}"),
            Instruction::LW { imm, rs1, rd } => write!(f, "LW x{rd} x{rs1} #{imm:#x}"),
            Instruction::LBU { imm, rs1, rd } => write!(f, "LBU x{rd} x{rs1} #{imm:#x}"),
            Instruction::LHU { imm, rs1, rd } => write!(f, "LHU x{rd} x{rs1} #{imm:#x}"),
            Instruction::SB { imm, rs2, rs1 } => write!(f, "SB x{rs1} x{rs2} #{imm:#x}"),
            Instruction::SH { imm, rs2, rs1 } => write!(f, "SH x{rs1} x{rs2} #{imm:#x}"),
            Instruction::SW { imm, rs2, rs1 } => write!(f, "SW x{rs1} x{rs2} #{imm:#x}"),
            Instruction::ADDI { imm, rs1, rd } => write!(f, "ADDI x{rd} x{rs1} #{imm:#x}"),
            Instruction::SLTI { imm, rs1, rd } => write!(f, "SLTI x{rd} x{rs1} #{imm:#x}"),
            Instruction::SLTIU { imm, rs1, rd } => write!(f, "SLTIU x{rd} x{rs1} #{imm:#x}"),
            Instruction::XORI { imm, rs1, rd } => write!(f, "XORI x{rd} x{rs1} #{imm:#x}"),
            Instruction::ORI { imm, rs1, rd } => write!(f, "ORI x{rd} x{rs1} #{imm:#x}"),
            Instruction::ANDI { imm, rs1, rd } => write!(f, "ANDI x{rd} x{rs1} #{imm:#x}"),
            Instruction::SLLI { shamt, rs1, rd } => write!(f, "SLLI x{rd} x{rs1} {shamt}"),
            Instruction::SRLI { shamt, rs1, rd } => write!(f, "SRLI x{rd} x{rs1} {shamt}"),
            Instruction::SRAI { shamt, rs1, rd } => write!(f, "SRAI x{rd} x{rs1} {shamt}"),
            Instruction::ADD { rs2, rs1, rd } => write!(f, "ADD x{rd} x{rs1} x{rs2}"),
            Instruction::SUB { rs2, rs1, rd } => write!(f, "SUB x{rd} x{rs1} x{rs2}"),
            Instruction::SLL { rs2, rs1, rd } => write!(f, "SLL x{rd} x{rs1} x{rs2}"),
            Instruction::SLT { rs2, rs1, rd } => write!(f, "SLT x{rd} x{rs1} x{rs2}"),
            Instruction::SLTU { rs2, rs1, rd } => write!(f, "SLTU x{rd} x{rs1} x{rs2}"),
            Instruction::XOR { rs2, rs1, rd } => write!(f, "XOR x{rd} x{rs1} x{rs2}"),
            Instruction::SRL { rs2, rs1, rd } => write!(f, "SRL x{rd} x{rs1} x{rs2}"),
            Instruction::SRA { rs2, rs1, rd } => write!(f, "SRA x{rd} x{rs1} x{rs2}"),
            Instruction::OR { rs2, rs1, rd } => write!(f, "OR x{rd} x{rs1} x{rs2}"),
            Instruction::AND { rs2, rs1, rd } => write!(f, "AND x{rd} x{rs1} x{rs2}"),
            Instruction::FENCE {
                fm,
                pred,
                succ,
                rs1,
                rd,
            } => todo!(),
            Instruction::ECALL => todo!(),
            Instruction::EBREAK => todo!(),
        }
    }
}

// At the moment I don't actually simulate these
pub(crate) enum MicroSteps {
    AluOp0,     // (alu operation 0)
    MemStore,   // (memory store - RAM)
    RtbSel0,    // (return bus select 0)
    PcSrc0,     // (program counter select source 0)
    RtbSel1,    // (return bus select 1)
    AliOp1,     // (alu operation 1)
    TakeBranch, // (conditional branch)
    PcSrc1,     // (program counter select source 0)
    ImmGenCtr0, // (immediate generator control 0)
    ImmGenCtr1, // (immediate generator control 1)
    RfStore,    // (register file store)
    AluG,       // (alu gate)
    PCE,        // (program counter enable)
    Reserved0,  // (reserved - nothing)
    AluSrc,     // (alu source)
    Reserved1,  // (reserved - nothing)
    ShLoad,     // (shifter load)
    ShLatch,    // (shifter latch)
    RfLoad,     // (register file load)
    MemLoad,    // (memory load - RAM)
    ImmGenCtr2, // (immediate generator control 2)
}

enum Immediate {
    I,
    S,
    B,
    U,
    J,
}

impl Immediate {
    fn from_i32(imm: Immediate, data: i32) -> i32 {
        match imm {
            Immediate::I => {
                extract_bits!(@extend data[31;20])
            }
            Immediate::S => {
                let inst_31 = extract_bits!(@extend data[31;31]);
                let inst_30_25 = extract_bits!(data[30;25]);
                let inst_11_8 = extract_bits!(data[11;8]);
                let inst_7 = extract_bits!(data[7]);
                (inst_31 << 11) | (inst_30_25 << 5) | (inst_11_8 << 1) | inst_7
            }
            Immediate::B => {
                let inst_31 = extract_bits!(@extend data[31]);
                let inst_7 = extract_bits!(data[7]);
                let inst_30_25 = extract_bits!(data[30;25]);
                let inst_11_8 = extract_bits!(data[11;8]);
                (inst_31 << 12) | (inst_7 << 11) | (inst_30_25 << 5) | (inst_11_8 << 1)
            }
            Immediate::U => {
                let inst_31 = extract_bits!(@extend data[31]);
                let inst_30_20 = extract_bits!(data[30;20]);
                let inst_19_12 = extract_bits!(data[19;12]);
                (inst_31 << 31) | (inst_30_20 << 20) | (inst_19_12 << 12)
            }
            Immediate::J => {
                let inst_31 = extract_bits!(@extend data[31]);
                let inst_19_12 = extract_bits!(data[19;12]);
                let inst_20 = extract_bits!(data[20]);
                let inst_30_25 = extract_bits!(data[30;25]);
                let inst_24_21 = extract_bits!(data[24;21]);
                (inst_31 << 20)
                    | (inst_19_12 << 12)
                    | (inst_20 << 11)
                    | (inst_30_25 << 5)
                    | (inst_24_21 << 1)
            }
        }
    }
}

//macro_rules! generate_imm {
//    ($data:ident[$bits(*$start:expr;$end:expr)*]) => {};
//}

impl Instruction {
    // TODO: This could **very** easily be turned into a macro. I'm too lazy to do so.
    pub fn from_i32(data: i32) -> Instruction {
        let opcode = extract_bits!(data[6;0]);
        match opcode {
            0b0110111 => Instruction::LUI {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            },
            0b0010111 => Instruction::AUIPC {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            },
            0b1101111 => Instruction::JAL {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            },
            0b1100111 => Instruction::JALR {
                imm: Immediate::from_i32(Immediate::I, data),
                rs1: Instruction::get_rs1(data),
                rd: Instruction::get_rd(data),
            },

            0b1100011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::BEQ {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b001 => Instruction::BNE {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b100 => Instruction::BLT {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b101 => Instruction::BGE {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b110 => Instruction::BLTU {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b111 => Instruction::BGEU {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }
            0b0000011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::LB {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b001 => Instruction::LH {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b010 => Instruction::LW {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b100 => Instruction::LBU {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b101 => Instruction::LHU {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }
            0b0100011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::SB {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b001 => Instruction::SH {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    0b010 => Instruction::SW {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    },
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }

            0b0010011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::ADDI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b010 => Instruction::SLTI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b011 => Instruction::SLTIU {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b100 => Instruction::XORI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b110 => Instruction::ORI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b111 => Instruction::ANDI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },
                    //Make sure you're getting the front zeros/1 from this point on
                    0b001 => Instruction::SLLI {
                        shamt: Instruction::get_shamt(data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    },

                    0b101 => {
                        let arethmetic_shift = extract_bits!(data[31]);
                        match arethmetic_shift {
                            0b0 => Instruction::SRLI {
                                shamt: Instruction::get_shamt(data),
                                rs1: Instruction::get_rs1(data),
                                rd: Instruction::get_rd(data),
                            },
                            0b1 => Instruction::SRAI {
                                shamt: Instruction::get_shamt(data),
                                rs1: Instruction::get_rs1(data),
                                rd: Instruction::get_rd(data),
                            },
                            _ => unreachable!("Impossible Bit!"),
                        }
                    }
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }
            0b0110011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => {
                        let direction_bit = extract_bits!(data[31]);
                        match direction_bit {
                            0b0 => Instruction::ADD {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            },
                            0b1 => Instruction::SUB {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            },
                            _ => unimplemented!("Unimplemented Opcode!"),
                        }
                    }
                    0b001 => Instruction::SLL {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b010 => Instruction::SLT {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b011 => Instruction::SLTU {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b100 => Instruction::XOR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b101 => {
                        let direction_bit = extract_bits!(data[31]);
                        match direction_bit {
                            0b0 => Instruction::SRL {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            },
                            0b1 => Instruction::SRA {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            },
                            _ => unimplemented!("Unimplemented Opcode!"),
                        }
                    }
                    0b110 => Instruction::OR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    0b111 => Instruction::AND {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    },
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }

            0b0001111 => unimplemented!("FENCE"),

            0b1110011 => {
                let call = extract_bits!(data[21]);
                match call {
                    0b0 => unimplemented!("ECALL"),
                    0b1 => unimplemented!("EBREAK"),
                    _ => unimplemented!("Impossible Bit!"),
                }
            }
            _ => unimplemented!("Unimplemented Opcode!"),
        }
    }

    fn get_rs1(data: i32) -> usize {
        extract_bits!(data[20;16]) as usize
    }

    fn get_rs2(data: i32) -> usize {
        extract_bits!(data[20;16]) as usize
    }

    fn get_rd(data: i32) -> usize {
        extract_bits!(data[12;8]) as usize
    }

    fn get_shamt(data: i32) -> i32 {
        Instruction::get_rs2(data) as i32
    }
}

#[cfg(test)]
#[allow(overflowing_literals)] // Needed because 0b1(x){31} is "overflowing"
mod tests {
    use super::*;

    // The spec defines 5 different immediate formats, each one has a different ordering for immediate values
    #[test]
    fn immediate_i() {
        // I-Type
        let inst_31 = 0b1111111111111111111111_00000_0000_0;
        let inst_30_25 = 0b0000000000000000000000_11111_0000_0;
        let inst_24_21 = 0b0000000000000000000000_00000_1111_0;
        let inst_20 = 0b0000000000000000000000_00000_0000_1;

        let imm_11_0 = 0b111111111111_00000_000_00000_0000000;
        let rs1 = 0b000000000000_11111_000_00000_0000000;
        let funct3 = 0b000000000000_00000_111_00000_0000000;
        let rd = 0b000000000000_00000_000_11111_0000000;
        let opcode = 0b000000000000_00000_000_00000_1111111;

        let non_imm = rs1 | funct3 | rd | opcode;
        assert_eq!(Immediate::from_i32(Immediate::I, non_imm), 0);

        assert_eq!(
            Immediate::from_i32(Immediate::I, imm_11_0),
            (inst_31 | inst_30_25 | inst_24_21 | inst_20)
        );
    }

    #[test]
    fn immediate_s() {
        // S-Type
        let inst_31 = 0b1111111111111111111111_00000_0000_0;
        let inst_30_25 = 0b0000000000000000000000_11111_0000_0;
        let inst_11_18 = 0b0000000000000000000000_00000_1111_0;
        let inst_7 = 0b0000000000000000000000_00000_0000_1;
        //11_11111_1111_1
        let imm_11_0 = 0b1111111_00000_00000_000_00000_0000000;
        let rs2 = 0b0000000_11111_00000_000_00000_0000000;
        let rs1 = 0b0000000_00000_11111_000_00000_0000000;
        let funct3 = 0b0000000_00000_00000_111_00000_0000000;
        let imm_4_0 = 0b0000000_00000_00000_000_11111_0000000;
        let opcode = 0b0000000_00000_00000_000_00000_1111111;

        let non_imm = rs2 | rs1 | funct3 | opcode;
        assert_eq!(Immediate::from_i32(Immediate::S, non_imm), 0);

        assert_eq!(
            Immediate::from_i32(Immediate::S, imm_11_0 | imm_4_0),
            (inst_31 | inst_30_25 | inst_11_18 | inst_7)
        );
    }

    #[test]
    fn immediate_b() {
        // B-Type
        let inst_31 = 0b111111111111111111111_00000_0000_0_0;
        let inst_30_25 = 0b000000000000000000000_11111_0000_0_0;
        let inst_11_18 = 0b000000000000000000000_00000_1111_0_0;
        let inst_7 = 0b000000000000000000000_00000_0000_1_0;
        //0b11_11111_1111_10
        let imm_12 = 0b1_000000_00000_00000_000_0000_0_0000000;
        let imm_10_5 = 0b0_111111_00000_00000_000_0000_0_0000000;
        let rs2 = 0b0_000000_11111_00000_000_0000_0_0000000;
        let rs1 = 0b0_000000_00000_11111_000_0000_0_0000000;
        let funct3 = 0b0_000000_00000_00000_111_0000_0_0000000;
        let imm_4_0 = 0b0_000000_00000_00000_000_1111_0_0000000;
        let imm_11 = 0b0_000000_00000_00000_000_0000_1_0000000;
        let opcode = 0b0_000000_00000_00000_000_0000_0_1111111;

        let non_imm = rs2 as i32 | rs1 as i32 | funct3 | opcode;
        assert_eq!(Immediate::from_i32(Immediate::B, non_imm), 0);

        assert_eq!(
            Immediate::from_i32(Immediate::B, imm_12 | imm_10_5 | imm_4_0 | imm_11),
            (inst_31 | inst_30_25 | inst_11_18 | inst_7)
        );
    }

    #[test]
    fn immediate_u() {
        let inst_31 = 0b1_00000000000_00000000_000000000000;
        let inst_30_20 = 0b0_11111111111_00000000_000000000000;
        let inst_19_12 = 0b0_00000000000_11111111_000000000000;

        let imm_31_12 = 0b11111111111111111111_00000_0000000;
        let rd = 0b00000000000000000000_11111_0000000;
        let opcode = 0b00000000000000000000_00000_1111111;

        let non_imm = rd | opcode;
        assert_eq!(Immediate::from_i32(Immediate::U, non_imm), 0);

        assert_eq!(
            Immediate::from_i32(Immediate::U, imm_31_12),
            (inst_31 | inst_30_20 | inst_19_12)
        )
    }
}
