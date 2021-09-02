#![feature(format_args_capture)]

use std::sync::RwLock;

use instruction::Instruction;
#[macro_use]
mod instruction;
mod memory;
mod process;

pub struct Pineapple {
    // For RISCV general_register[0] always equals 0
    // The PC is always XLEN-1
    general_register: RwLock<Vec<i32>>,
    program_counter: RwLock<usize>,
    instruction_memory: RwLock<Vec<i32>>,
    data_memory: memory::MemorySystem,
}

impl Pineapple {
    pub fn new() -> Self {
        Pineapple {
            program_counter: RwLock::new(0),
            // 0x13 is NOOP
            instruction_memory: RwLock::new(vec![0x13; 524_288]),
            general_register: RwLock::new(vec![0; 32]),
            data_memory: memory::MemorySystem::new(),
        }
    }

    pub async fn get_video_memory(&self) -> Result<Vec<i32>, ()> {
        // Lazy
        self.data_memory.dump_memory_range(0x40000000, 0x400007FF)
    }

    pub async fn get_program_counter(&self) -> Result<usize, ()> {
        let lock = self.program_counter.read();
        Ok(*lock.unwrap_or(Err(())?))
    }

    pub async fn get_registers(&self) -> Result<Vec<i32>, ()> {
        let lock = self.general_register.read();
        Ok(lock.unwrap_or(Err(())?).clone())
    }

    pub async fn get_data_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
        self.data_memory.dump_memory_range(start, stop)
    }

    pub async fn get_instruction_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
        let memory = match self.instruction_memory.try_read() {
            Ok(rw_lock) => rw_lock,
            Err(_) => todo!(),
        };
        let mut result: Vec<i32> = Vec::new();
        result.reserve(stop - start);

        for n in start..stop {
            result.push(*memory.get(n).ok_or(Err(())?)?);
        }
        Ok(result)
    }

    pub fn set_program(&mut self, memory: &[i32], start: usize) {
        // TODO: RWLock stuff.
        let mut instruction_memory = self.instruction_memory.write().unwrap();

        if memory.len() + start > instruction_memory.len() {
            // TODO: Surely there's a better way of doing this
            panic!("Tried to address memory out of bounds!")
        }
        for (idx, value) in memory.iter().enumerate() {
            let real_address = start + idx;
            instruction_memory[real_address] = *value;
        }
    }

    pub fn step(&mut self) -> Instruction {
        let addr = *self.program_counter.read().unwrap();
        let instr = Instruction::from_i32(self.instruction_memory.read().unwrap()[addr]);
        self.process_instruction(&instr);
        instr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        println!("Test");
        let mut pineapple = Pineapple::new();
        for _ in 0..5 {
            let instruction= pineapple.step();
            println!("{}",instruction)
        }
    }
}
