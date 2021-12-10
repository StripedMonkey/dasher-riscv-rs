use byteorder::{ByteOrder, LittleEndian};
use std::{sync::RwLock, vec};
pub(crate) struct MemorySystem {
    ram: Vec<i8>,
    video_memory: Vec<i8>,
}

impl MemorySystem {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 524_288 * 4],
            video_memory: vec![0; 2048 * 4],
        }
    }
    pub fn read_i32(&self, idx: usize) -> i32 {
        match idx {
            0x00000000..=0x0001FFFF => {
                // RAM
                let contents = &self.ram;
                read_slice(&contents[idx..(idx + 5)])
            }
            0x00020000..=0x3FFFFFFF => {
                // Reserved Space
                unimplemented!("Reserved Space is Unimplemented!")
            }
            0x40000000..=0x400007FF => {
                // Video RAM
                let contents = &self.ram;
                read_slice(&contents[idx..(idx + 5)])
            }
            0x40000800..=0x7FFFFFFF => {
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
                let mut contents = &mut self.ram;
                write_slice(&mut contents[idx..(idx + 5)], data);
            }
            0x00020000..=0x3FFFFFFF => {
                // Reserved Space
                unimplemented!("Attempted to write out of bounds!")
            }
            0x40000000..=0x400007FF => {
                // Video RAM
                let mut contents = &mut self.video_memory;
                write_slice(&mut contents[idx..(idx + 5)], data);
            }
            0x40000800..=0x7FFFFFFF => {
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
            let contents = &self.ram;
            for i in (start..stop).step_by(4) {
                dump.push(read_slice(&contents[i..(i + 5)]));
            }
            return Ok(dump);
        }
        if start < 0x40000000 && stop < 0x40000000 {
            let contents = &self.video_memory;
            for i in (start..stop).step_by(4) {
                dump.push(read_slice(&contents[i..(i + 5)]));
            }
            return Ok(dump);
        }
        Err(())
    }
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self::new()
    }
}

fn read_slice(slice: &[i8]) -> i32 {
    let u8slice = conv(slice);

    LittleEndian::read_i32(u8slice)
}

fn write_slice(slice: &mut [i8], contents: i32) {
    LittleEndian::write_i32(conv_mut(slice), contents);
}

fn conv_mut<'a>(p: &'a mut [i8]) -> &'a mut [u8] {
    // Safety: this is fine since they're equivilant size/shapes
    unsafe {
        &mut *(p as *mut [i8] as *mut [u8])
    }
}
fn conv<'a>(p: &'a [i8]) -> &'a [u8] {
    // Safety: this is fine since they're equivilant size/shapes
    unsafe {
        &*(p as *const [i8] as *const [u8])
    }
}
