#![feature(format_args_capture)]

use std::sync::RwLock;
#[macro_use]
mod instruction;
mod process;

pub struct Pineapple {
    // For RISCV general_register[0] always equals 0
    // The PC is always XLEN-1
    general_register: RwLock<[i32; 32]>,
    program_counter: RwLock<i32>,
    instruction_memory: RwLock<[i32; 524_288]>,
    data_memory: MemorySystem,
}

struct MemorySystem {
    ram: RwLock<[i32; 524_288]>,
    video_memory: RwLock<[i32; 2_048]>,
}

impl MemorySystem {
    fn read_i32(&self, idx: usize) -> i32 {
        match idx {
            0x00000000..=0x0001FFFF => {
                // RAM
                let contents = self.ram.read().unwrap();
                contents[idx]
            }
            0x00020000..=0x3FFFFFFF => {
                // Reserved Space
                unimplemented!("Reserved Space is Unimplemented!")
            }
            0x40000000..=0x400007FF => {
                // Video RAM
                let contents = self.ram.read().unwrap();
                contents[idx]
            }
            0x40000000..=0x7FFFFFFF => {
                // Reserved Space
                unimplemented!("Reserved Space is Unimplemented!")
            }
            0x80000000..=0x8000000F => {
                // Special Registry
                unimplemented!("Special registry is unimplemented!")
            }
            0x80000010..=0xFFFFFFFF => {
                // Reserved Space
                unimplemented!("Reserved Space is Unimplemented!")
            }
            _ => unimplemented!("Out of range!"),
        }
    }

    fn write_i32(&mut self, idx: usize, data: i32) {
        match idx {
            0x00000000..=0x0001FFFF => {
                // RAM
                let mut contents = self.ram.write().unwrap();
                contents[idx] = data;
            }
            0x00020000..=0x3FFFFFFF => {
                // Reserved Space
                unimplemented!("Attempted to write out of bounds!")
            }
            0x40000000..=0x400007FF => {
                // Video RAM
                let mut contents = self.video_memory.write().unwrap();
                contents[idx] = data;
            }
            0x40000000..=0x7FFFFFFF => {
                // Reserved Space
                unimplemented!("Attempted to write out of bounds!")
            }
            0x80000000..=0x8000000F => {
                // Special Registry
                unimplemented!("Attempted to write out of bounds!")
            }
            0x80000010..=0xFFFFFFFF => {
                // Reserved Space
                unimplemented!("Attempted to write out of bounds!")
            }
            _ => {
                unimplemented!("Attempted to write out of bounds!")
            }
        }
    }

    fn dump_memory_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
        // There's probably a more eloquent way to write this
        let mut dump: Vec<i32> = Vec::new();
        if start < 0x00020000 && stop < 0x00020000 {
            let contents = self.ram.read().unwrap();
            for i in start..stop {
                dump.push(contents[i]);
            }
            return Ok(dump);
        }
        if start < 0x40000000 && stop < 0x40000000 {
            let contents = self.video_memory.read().unwrap();
            for i in start..stop {
                dump.push(contents[i])
            }
            return Ok(dump);
        }
        Err(())
    }
}

impl Pineapple {
    pub fn new() -> Self {
        Pineapple {
            program_counter: RwLock::new(0),
            instruction_memory: RwLock::new([0; 524_288]),
            general_register: RwLock::new([0; 32]),
            data_memory: MemorySystem {
                ram: RwLock::new([0; 524_288]),
                video_memory: RwLock::new([0; 2048]),
            },
        }
    }

    pub async fn get_video_memory(&self) -> Result<Vec<i32>, ()> {
        // Lazy
        self.data_memory.dump_memory_range(0x40000000, 0x400007FF)
    }

    pub async fn get_program_counter(&self) -> Result<i32, ()> {
        let lock = self.program_counter.read();
        Ok(*lock.unwrap_or(Err(())?))
    }

    pub async fn get_registers(&self) -> Result<[i32; 32], ()> {
        let lock = self.general_register.read();
        Ok(*lock.unwrap_or(Err(())?))
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
}

#[cfg(test)]
mod tests {}
