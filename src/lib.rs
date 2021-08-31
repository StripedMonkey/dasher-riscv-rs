#![feature(format_args_capture)]
mod instruction;

pub struct Pineapple {
    // For RISCV general_register[0] always equals 0
    // The PC is always XLEN-1
    general_register: [i32; 32],
    program_counter: i32,
    instruction_memory: [i32; 524_288],
    data_memory: [i32; 524_288],
    video_memory: [i32; 2_048],
}

impl Pineapple {
    pub fn new() -> Self {
        Pineapple {
            program_counter: 0,
            instruction_memory: [0; 524_288],
            data_memory: [0; 524_288],
            video_memory: [0; 2048],
            general_register: [0; 32],
        }
    }
    
    pub fn get_registers(&self) -> [i32;32] {
        self.general_register
    }

    pub fn get_data_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
        let mut result: Vec<i32> = Vec::new();
        result.reserve(stop - start);
        for n in start..stop {
            result.push(*self.data_memory.get(n).ok_or(Err(())?)?);
        }
        Ok(result)
    }
    pub fn get_instruction_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
        let mut result: Vec<i32> = Vec::new();
        result.reserve(stop - start);
        for n in start..stop {
            result.push(*self.instruction_memory.get(n).ok_or(Err(())?)?);
        }
        Ok(result)
    }

    pub fn set_program(&mut self, memory: &[i32], start: usize) {
        if memory.len() + start > self.instruction_memory.len() {
            // TODO: Surely there's a better way of doing this
            panic!("Tried to address memory out of bounds!")
        }
        for (idx, value) in memory.iter().enumerate() {
            let real_address = start + idx;
            self.instruction_memory[real_address] = *value;
        }
    }
}

#[cfg(test)]
mod tests {}
