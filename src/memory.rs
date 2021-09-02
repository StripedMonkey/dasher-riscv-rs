use std::{sync::RwLock, vec};

pub(crate) struct MemorySystem {
    ram: RwLock<Vec<i32>>,
    video_memory: RwLock<Vec<i32>>,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            ram: RwLock::new(vec![0; 524_288]),
            video_memory: RwLock::new(vec![0; 2048]),
        }
    }
    pub fn read_i32(&self, idx: usize) -> i32 {
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

    pub fn write_i32(&mut self, idx: usize, data: i32) {
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

    pub fn dump_memory_range(&self, start: usize, stop: usize) -> Result<Vec<i32>, ()> {
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
