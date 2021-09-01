use crate::instruction::sign_extend;
use crate::{instruction::Instruction, Pineapple};

impl Pineapple {
    fn process_instruction(&mut self, instruction: Instruction) {
        let mut registers = self.general_register.write().unwrap();
        match instruction {
            Instruction::LUI { imm, rd } => todo!(),
            Instruction::AUIPC { imm, rd } => todo!(),
            Instruction::JAL { imm, rd } => todo!(),
            Instruction::JALR { imm, rs1, rd } => todo!(),
            Instruction::BEQ { imm, rs2, rs1 } => todo!(),
            Instruction::BNE { imm, rs2, rs1 } => todo!(),
            Instruction::BLT { imm, rs2, rs1 } => todo!(),
            Instruction::BGE { imm, rs2, rs1 } => todo!(),
            Instruction::BLTU { imm, rs2, rs1 } => todo!(),
            Instruction::BGEU { imm, rs2, rs1 } => todo!(),
            Instruction::LB { imm, rs1, rd } => {
                let offset = (registers[rs1] + imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[rd] = extract_bits!(@extend data[7;0]);
            }
            Instruction::LH { imm, rs1, rd } => {
                let offset = (registers[rs1] + imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[rd] = extract_bits!(@extend data[15;0]);
            }
            Instruction::LW { imm, rs1, rd } => {
                registers[rd] = self.data_memory.read_i32((registers[rs1]+imm) as usize)
            }
            Instruction::LBU { imm, rs1, rd } => {
                let offset = (registers[rs1] + imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[rd] = extract_bits!(data[15;0]);
            }
            Instruction::LHU { imm, rs1, rd } => {
                let destination = (registers[rs1] + imm) as usize;
                let data = self.data_memory.read_i32(destination);
                registers[rd] = extract_bits!(data[15;0]);
            }
            Instruction::SB { imm, rs2, rs1 } => {
                let destination = (registers[rs1] + imm) as usize;
                self.data_memory
                    .write_i32(destination, registers[rs2] & 0xFF);
            }
            Instruction::SH { imm, rs2, rs1 } => {
                let destination = (registers[rs1] + imm) as usize;
                self.data_memory
                    .write_i32(destination, registers[rs2] & 0xFFFF);
            }
            Instruction::SW { imm, rs2, rs1 } => {
                let destination = (registers[rs1] + imm) as usize;
                self.data_memory.write_i32(destination, registers[rs2]);
            }
            Instruction::ADDI { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                let (value, _) = registers[rs1].overflowing_add(imm);
                registers[rd] = value;
            }
            Instruction::SLTI { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd as usize] = match registers[rs1] < imm {
                    true => 1,
                    false => 0,
                };
            }
            Instruction::SLTIU { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd as usize] = match (registers[rs1] as u32) < (imm as u32) {
                    true => 1,
                    false => 0,
                };
            }
            Instruction::XORI { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] ^ imm;
            }
            Instruction::ORI { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] | imm;
            }
            Instruction::ANDI { imm, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] & imm;
            }
            Instruction::SLLI { shamt, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] << shamt;
            }
            Instruction::SRLI { shamt, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = ((registers[rs1] as u32) >> shamt) as i32;
            }
            Instruction::SRAI { shamt, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] >> shamt;
            }
            Instruction::ADD { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                let (result, _) = registers[rs1].overflowing_add(registers[rs2]);
                registers[rd] = result;
            }
            Instruction::SUB { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                let (result, _) = registers[rs1].overflowing_sub(registers[rs2]);
                registers[rd] = result;
            }
            Instruction::SLL { rs2, rs1, rd } => {
                // Lower five bits
                registers[rd] = registers[rs1] << (registers[rs2] & 0x1F)
            }
            Instruction::SLT { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = match registers[rs1] < registers[rs2] {
                    true => 1,
                    false => 0,
                }
            }
            Instruction::SLTU { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = match (registers[rs1] as u32) < (registers[rs2] as u32) {
                    true => 1,
                    false => 0,
                }
            }
            Instruction::XOR { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] ^ registers[rs2];
            }
            Instruction::SRL { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                // Specifically uses the lower 5 bits only
                registers[rd] = (registers[rs1] as u32 >> (registers[rs2] & 0b11111)) as i32
            }
            Instruction::SRA { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                // Specifically uses the lower 5 bits only
                registers[rd] = registers[rs1] >> (registers[rs2] & 0b11111)
            }
            Instruction::OR { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] | registers[rs2];
            }
            Instruction::AND { rs2, rs1, rd } => {
                if rd == 0 {
                    return;
                }
                registers[rd] = registers[rs1] & registers[rs2];
            }
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
