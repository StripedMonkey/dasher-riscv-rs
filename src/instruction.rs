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

// TODO: Eventually Enum Variants will be their own proper types, when that happens this can be folded into a single enum
pub struct InstructionTypeR {
    pub rs2: usize,
    pub rs1: usize,
    pub rd: usize,
}
pub struct InstructionTypeI {
    pub imm: i32,
    pub rs1: usize,
    pub rd: usize,
}
pub struct InstructionTypeS {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
}
pub struct InstructionTypeB {
    pub imm: i32,
    pub rs2: usize,
    pub rs1: usize,
}
pub struct InstructionTypeU {
    pub imm: i32,
    pub rd: usize,
}

#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    LUI(InstructionTypeU),
    AUIPC(InstructionTypeU),
    JAL(InstructionTypeU),
    JALR(InstructionTypeI),
    BEQ(InstructionTypeB),
    BNE(InstructionTypeB),
    BLT(InstructionTypeB),
    BGE(InstructionTypeB),
    BLTU(InstructionTypeB),
    BGEU(InstructionTypeB),
    LB(InstructionTypeI),
    LH(InstructionTypeI),
    LW(InstructionTypeI),
    LBU(InstructionTypeI),
    LHU(InstructionTypeI),
    SB(InstructionTypeS),
    SH(InstructionTypeS),
    SW(InstructionTypeS),
    ADDI(InstructionTypeI),
    SLTI(InstructionTypeI),
    SLTIU(InstructionTypeI),
    XORI(InstructionTypeI),
    ORI(InstructionTypeI),
    ANDI(InstructionTypeI),
    SLLI(InstructionTypeI),
    SRLI(InstructionTypeI),
    SRAI(InstructionTypeI),
    ADD(InstructionTypeR),
    SUB(InstructionTypeR),
    SLL(InstructionTypeR),
    SLT(InstructionTypeR),
    SLTU(InstructionTypeR),
    XOR(InstructionTypeR),
    SRL(InstructionTypeR),
    SRA(InstructionTypeR),
    OR(InstructionTypeR),
    AND(InstructionTypeR),
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
            Instruction::LUI(i) => write!(f, "LUI x{} #{:#x}", i.rd, i.imm),

            Instruction::AUIPC(i) => write!(f, "AUIPC x{} #{:#x}", i.rd, i.imm),
            Instruction::JAL(i) => write!(f, "JAL x{} #{:#x}", i.rd, i.imm),
            Instruction::JALR(i) => write!(f, "JALR x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::BEQ(i) => write!(f, "BEQ x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::BNE(i) => write!(f, "BNE x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::BLT(i) => write!(f, "BLT x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::BGE(i) => write!(f, "BGE x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::BLTU(i) => write!(f, "BLTU x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::BGEU(i) => write!(f, "BGEU x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::LB(i) => write!(f, "LB x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::LH(i) => write!(f, "LH x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::LW(i) => write!(f, "LW x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::LBU(i) => write!(f, "LBU x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::LHU(i) => write!(f, "LHU x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::SB(i) => write!(f, "SB x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::SH(i) => write!(f, "SH x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::SW(i) => write!(f, "SW x{} x{} #{:#x}", i.rs1, i.rs2, i.imm),
            Instruction::ADDI(i) => write!(f, "ADDI x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::SLTI(i) => write!(f, "SLTI x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::SLTIU(i) => write!(f, "SLTIU x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::XORI(i) => write!(f, "XORI x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::ORI(i) => write!(f, "ORI x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::ANDI(i) => write!(f, "ANDI x{} x{} #{:#x}", i.rd, i.rs1, i.imm),
            Instruction::SLLI(i) => write!(f, "SLLI x{} x{} {}", i.rd, i.rs1, i.imm),
            Instruction::SRLI(i) => write!(f, "SRLI x{} x{} {}", i.rd, i.rs1, i.imm),
            Instruction::SRAI(i) => write!(f, "SRAI x{} x{} {}", i.rd, i.rs1, i.imm),
            Instruction::ADD(i) => write!(f, "ADD x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SUB(i) => write!(f, "SUB x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SLL(i) => write!(f, "SLL x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SLT(i) => write!(f, "SLT x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SLTU(i) => write!(f, "SLTU x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::XOR(i) => write!(f, "XOR x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SRL(i) => write!(f, "SRL x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::SRA(i) => write!(f, "SRA x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::OR(i) => write!(f, "OR x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::AND(i) => write!(f, "AND x{} x{} x{}", i.rd, i.rs1, i.rs2),
            Instruction::FENCE {
                fm,
                pred,
                succ,
                rs1,
                rd,
            } => {
                unimplemented!(
                    "Unimplemented instruction! {}{}{}{}{}",
                    fm,
                    pred,
                    succ,
                    rs1,
                    rd
                )
            }
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
    //#[deprecated = "J type Immediate values are depreciated!"]
    //J,
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
            /* 
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
            */
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
            0b0110111 => Instruction::LUI(InstructionTypeU {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            }),
            0b0010111 => Instruction::AUIPC(InstructionTypeU {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            }),
            0b1101111 => Instruction::JAL(InstructionTypeU {
                imm: Immediate::from_i32(Immediate::U, data),
                rd: Instruction::get_rd(data),
            }),
            0b1100111 => Instruction::JALR(InstructionTypeI {
                imm: Immediate::from_i32(Immediate::I, data),
                rs1: Instruction::get_rs1(data),
                rd: Instruction::get_rd(data),
            }),

            0b1100011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::BEQ(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b001 => Instruction::BNE(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b100 => Instruction::BLT(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b101 => Instruction::BGE(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b110 => Instruction::BLTU(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b111 => Instruction::BGEU(InstructionTypeB {
                        imm: Immediate::from_i32(Immediate::B, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }
            0b0000011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::LB(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b001 => Instruction::LH(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b010 => Instruction::LW(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b100 => Instruction::LBU(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b101 => Instruction::LHU(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }
            0b0100011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::SB(InstructionTypeS {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b001 => Instruction::SH(InstructionTypeS {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    0b010 => Instruction::SW(InstructionTypeS {
                        imm: Immediate::from_i32(Immediate::S, data),
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                    }),
                    _ => unimplemented!("Unimplemented Opcode!"),
                }
            }

            0b0010011 => {
                let secondary_opcode = extract_bits!(data[14;12]);
                match secondary_opcode {
                    0b000 => Instruction::ADDI(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b010 => Instruction::SLTI(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b011 => Instruction::SLTIU(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b100 => Instruction::XORI(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b110 => Instruction::ORI(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b111 => Instruction::ANDI(InstructionTypeI {
                        imm: Immediate::from_i32(Immediate::I, data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),
                    //Make sure you're getting the front zeros/1 from this point on
                    0b001 => Instruction::SLLI(InstructionTypeI {
                        imm: Instruction::get_shamt(data),
                        rs1: Instruction::get_rs1(data),
                        rd: Instruction::get_rd(data),
                    }),

                    0b101 => {
                        let arethmetic_shift = extract_bits!(data[31]);
                        match arethmetic_shift {
                            0b0 => Instruction::SRLI(InstructionTypeI {
                                imm: Instruction::get_shamt(data),
                                rs1: Instruction::get_rs1(data),
                                rd: Instruction::get_rd(data),
                            }),
                            0b1 => Instruction::SRAI(InstructionTypeI {
                                imm: Instruction::get_shamt(data),
                                rs1: Instruction::get_rs1(data),
                                rd: Instruction::get_rd(data),
                            }),
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
                            0b0 => Instruction::ADD(InstructionTypeR {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            }),
                            0b1 => Instruction::SUB(InstructionTypeR {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            }),
                            _ => unimplemented!("Unimplemented Opcode!"),
                        }
                    }
                    0b001 => Instruction::SLL(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b010 => Instruction::SLT(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b011 => Instruction::SLTU(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b100 => Instruction::XOR(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b101 => {
                        let direction_bit = extract_bits!(data[31]);
                        match direction_bit {
                            0b0 => Instruction::SRL(InstructionTypeR {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            }),
                            0b1 => Instruction::SRA(InstructionTypeR {
                                rs1: Instruction::get_rs1(data),
                                rs2: Instruction::get_rs2(data),
                                rd: Instruction::get_rd(data),
                            }),
                            _ => unimplemented!("Unimplemented Opcode!"),
                        }
                    }
                    0b110 => Instruction::OR(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
                    0b111 => Instruction::AND(InstructionTypeR {
                        rs1: Instruction::get_rs1(data),
                        rs2: Instruction::get_rs2(data),
                        rd: Instruction::get_rd(data),
                    }),
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
