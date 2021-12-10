use crate::instruction::sign_extend;
use crate::{instruction::Instruction, Pineapple};

impl Pineapple {
    pub(crate) fn process_instruction(&mut self, instruction: &Instruction) {
        let mut registers = self.general_register.write().unwrap();
        let mut pc = self.program_counter.write().unwrap();
        match instruction {
            Instruction::LUI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = i.imm;
            }
            Instruction::AUIPC (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = *pc as i32 + i.imm;
            }
            Instruction::JAL (i) => {
                let target_address = *pc as i32 + i.imm;
                if i.rd != 0 {
                    registers[i.rd] = *pc as i32 + 1
                }
                *pc = target_address as usize;
            }
            Instruction::JALR (i) => {
                let target_address = (registers[i.rs1] + i.imm) & -2;
                if i.rd != 0 {
                    registers[i.rd] = *pc as i32 + 1
                }
                *pc = target_address as usize;
            }
            Instruction::BEQ (i) => {
                if registers[i.rs1] == registers[i.rs2] {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::BNE (i) => {
                if registers[i.rs1] != registers[i.rs2] {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::BLT (i) => {
                if registers[i.rs1] < registers[i.rs2] {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::BGE (i) => {
                if registers[i.rs1] >= registers[i.rs2] {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::BLTU (i) => {
                if (registers[i.rs1] as u32) < (registers[i.rs2] as u32) {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::BGEU (i) => {
                if (registers[i.rs1] as u32) >= (registers[i.rs2] as u32) {
                    let (result, _) = (*pc as i32).overflowing_add(i.imm);
                    *pc = result as usize;
                }
            }
            Instruction::LB (i) => {
                let offset = (registers[i.rs1] +i.imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[i.rd] = extract_bits!(@extend data[7;0]);
            }
            Instruction::LH (i) => {
                let offset = (registers[i.rs1] +i.imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[i.rd] = extract_bits!(@extend data[15;0]);
            }
            Instruction::LW (i) => {
                registers[i.rd] = self.data_memory.read_i32((registers[i.rs1] +i.imm) as usize)
            }
            Instruction::LBU (i) => {
                let offset = (registers[i.rs1] +i.imm) as usize;
                let data = self.data_memory.read_i32(offset);
                registers[i.rd] = extract_bits!(data[15;0]);
            }
            Instruction::LHU (i) => {
                let destination = (registers[i.rs1] +i.imm) as usize;
                let data = self.data_memory.read_i32(destination);
                registers[i.rd] = extract_bits!(data[15;0]);
            }
            Instruction::SB (i) => {
                let destination = (registers[i.rs1] +i.imm) as usize;
                self.data_memory
                    .write_i32(destination, registers[i.rs2] & 0xFF);
            }
            Instruction::SH (i) => {
                let destination = (registers[i.rs1] +i.imm) as usize;
                self.data_memory
                    .write_i32(destination, registers[i.rs2] & 0xFFFF);
            }
            Instruction::SW (i) => {
                let destination = (registers[i.rs1] +i.imm) as usize;
                self.data_memory.write_i32(destination, registers[i.rs2]);
            }
            Instruction::ADDI (i) => {
                if i.rd == 0 {
                    return;
                }
                let (value, _) = registers[i.rs1].overflowing_add(i.imm);
                registers[i.rd] = value;
            }
            Instruction::SLTI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd as usize] = match registers[i.rs1] < i.imm {
                    true => 1,
                    false => 0,
                };
            }
            Instruction::SLTIU (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd as usize] = match (registers[i.rs1] as u32) < (i.imm as u32) {
                    true => 1,
                    false => 0,
                };
            }
            Instruction::XORI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] ^i.imm;
            }
            Instruction::ORI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] |i.imm;
            }
            Instruction::ANDI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] &i.imm;
            }
            Instruction::SLLI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] << i.imm;
            }
            Instruction::SRLI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = ((registers[i.rs1] as u32) >> i.imm) as i32;
            }
            Instruction::SRAI (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] >> i.imm;
            }
            Instruction::ADD (i) => {
                if i.rd == 0 {
                    return;
                }
                let (result, _) = registers[i.rs1].overflowing_add(registers[i.rs2]);
                registers[i.rd] = result;
            }
            Instruction::SUB (i) => {
                if i.rd == 0 {
                    return;
                }
                let (result, _) = registers[i.rs1].overflowing_sub(registers[i.rs2]);
                registers[i.rd] = result;
            }
            Instruction::SLL (i) => {
                // Lower five bits
                registers[i.rd] = registers[i.rs1] << (registers[i.rs2] & 0x1F)
            }
            Instruction::SLT (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = match registers[i.rs1] < registers[i.rs2] {
                    true => 1,
                    false => 0,
                }
            }
            Instruction::SLTU (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = match (registers[i.rs1] as u32) < (registers[i.rs2] as u32) {
                    true => 1,
                    false => 0,
                }
            }
            Instruction::XOR (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] ^ registers[i.rs2];
            }
            Instruction::SRL (i) => {
                if i.rd == 0 {
                    return;
                }
                // Specifically uses the lower 5 bits only
                registers[i.rd] = (registers[i.rs1] as u32 >> (registers[i.rs2] & 0b11111)) as i32
            }
            Instruction::SRA (i) => {
                if i.rd == 0 {
                    return;
                }
                // Specifically uses the lower 5 bits only
                registers[i.rd] = registers[i.rs1] >> (registers[i.rs2] & 0b11111)
            }
            Instruction::OR (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] | registers[i.rs2];
            }
            Instruction::AND (i) => {
                if i.rd == 0 {
                    return;
                }
                registers[i.rd] = registers[i.rs1] & registers[i.rs2];
            }
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
