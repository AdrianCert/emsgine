use std::cell::RefCell;

// use emsgine_lib::bitwise::{clr_bit, dword_split, set_bit, tgl_bit, word_split};
use emsgine_lib::contexts::Context;
// use errors::OverflowMemoryError;
// use std::rc::Rc;

/*
pub trait MemoryReadCapability {
    fn read_bytes(&self, addr: usize, bytes: u8) -> Vec<u8>;
    fn read_byte(&self, addr: usize) -> Result<u8, OverflowMemoryError>;
    fn read_word(&self, addr: usize) -> Result<u16, OverflowMemoryError> {
        return self.read_byte(addr).and_then(|msp| {
            return self.read_byte(addr + 1).and_then(|lsp| {
                let res = (msp as u16) << 8;
                return Ok(lsp as u16 + res);
            });
        });
    }
    fn read_dword(&self, addr: usize) -> Result<u32, OverflowMemoryError> {
        return self.read_word(addr).and_then(|msp| {
            return self.read_word(addr + 2).and_then(|lsp| {
                let res = (msp as u32) << 16;
                return Ok(lsp as u32 + res);
            });
        });
    }
    fn overflow_addr(&self) -> usize;
}

pub trait MemoryWriteCapability {
    fn write_byte(&self, addr: usize, value: u8) -> bool;
    fn write_word(&self, addr: usize, value: u16) -> bool {
        let (msb, lsb): (u8, u8) = word_split(value);
        return self.write_byte(addr, msb) && self.write_byte(addr + 1, lsb);
    }
    fn write_dword(&self, addr: usize, value: u32) -> bool {
        let (msw, lsw): (u16, u16) = dword_split(value);
        return self.write_word(addr, msw) && self.write_word(addr + 2, lsw);
    }
}

pub trait MemoryCapability: MemoryReadCapability + MemoryWriteCapability {}

#[derive(Debug)]
pub struct Register {
    value: Cell<u8>,
}

impl Default for Register {
    fn default() -> Self {
        return Self {
            value: Cell::new(0x00),
        };
    }
}

impl Register {
    pub fn new() -> Register {
        return Register {
            value: Cell::new(0x00),
        };
    }

    pub fn from(value: Option<u8>) -> Register {
        return Register {
            value: Cell::new(value.unwrap_or(0x00)),
        };
    }

    pub fn clr(&self, bit: u8) {
        self.value.set(clr_bit!(self.value.get(), bit));
    }

    pub fn set(&self, bit: u8) {
        self.value.set(set_bit!(self.value.get(), bit));
    }

    pub fn tgl(&self, bit: u8) {
        self.value.set(tgl_bit!(self.value.get(), bit));
    }
}

impl MemoryReadCapability for Register {
    fn read_bytes(&self, addr: usize, bytes: u8) -> Vec<u8> {
        // if let bytes = bytes.unwrap_or(0) > 0 {
        //     return Err(OverflowMemoryError{});
        // }
        return vec![self.value.get()];
    }

    fn read_byte(&self, addr: usize) -> Result<u8, OverflowMemoryError> {
        if addr == 0 {
            return Err(OverflowMemoryError {});
        }
        return Ok(self.value.get());
    }
    fn overflow_addr(&self) -> usize {
        return 1;
    }
}

impl MemoryWriteCapability for Register {
    fn write_byte(&self, addr: usize, value: u8) -> bool {
        if addr != 0 {
            return false;
        }
        self.value.set(value);
        return true;
    }
}

impl MemoryCapability for Register {}


impl<T> MemoryReadCapability for MemoryDevice<T> {
    fn read_byte(&self, addr: usize) -> Result<u8, OverflowMemoryError> {
        if addr >= self.overflow_addr() {
            return Err(OverflowMemoryError {});
        }
        return Ok(self.data.borrow()[addr]);
    }

    fn overflow_addr(&self) -> usize {
        return self.capacity;
    }
}

impl<T> MemoryWriteCapability for MemoryDevice<T> {
    fn write_byte(&self, addr: usize, value: u8) -> bool {
        self.data.borrow_mut()[addr] = value;
        return true;
    }
}

impl<T> MemoryCapability for MemoryDevice<T> {}

struct VirtualMemoryCompomet {
    device: Rc<dyn MemoryCapability>,
    offset: usize,
    capacity: usize,
}

pub struct VirtualMemoryDevice {
    data_map: Vec<VirtualMemoryCompomet>,
    capacity: usize,
}

impl VirtualMemoryDevice {
    pub fn new(devices: Vec<Rc<dyn MemoryCapability>>) -> VirtualMemoryDevice {
        let mut offset: usize = 0;
        let data_map: Vec<VirtualMemoryCompomet> = devices
        .iter()
        .map(|dev| {
            let curr_offset = offset;
                offset = curr_offset + dev.as_ref().overflow_addr();
                return VirtualMemoryCompomet {
                    device: dev.clone(),
                    capacity: dev.as_ref().overflow_addr(),
                    offset: curr_offset,
                };
            })
            .collect();

        return VirtualMemoryDevice {
            data_map,
            capacity: offset,
        };
    }

    fn dispatch_addr(
        &self,
        addr: usize,
    ) -> Result<(Rc<dyn MemoryCapability>, usize), OverflowMemoryError> {
        if addr >= self.capacity {
            return Err(OverflowMemoryError {});
        }
        // performing binary search to indentify the pair(device, real_address)
        let mut up_bound: usize = self.data_map.len() - 1;
        let mut lw_bound: usize = 0;
        while up_bound != lw_bound {
            let md_bound: usize = lw_bound + (up_bound - lw_bound) / 2;
            let item = &self.data_map[md_bound];
            if addr >= item.offset {
                let rel_addr = addr - item.offset;
                let item = &self.data_map[md_bound];
                if rel_addr < item.capacity {
                    return Ok((item.device.clone(), rel_addr));
                }
                lw_bound = md_bound + 1;
                continue;
            }
            up_bound = md_bound - 1
        }
        let item = &self.data_map[lw_bound];
        let rel_addr = addr - item.offset;
        return Ok((item.device.clone(), rel_addr));
    }
}

impl MemoryReadCapability for VirtualMemoryDevice {
    fn overflow_addr(&self) -> usize {
        return self.capacity;
    }

    fn read_byte(&self, addr: usize) -> Result<u8, OverflowMemoryError> {
        return self
        .dispatch_addr(addr)
        .and_then(|dips: (Rc<dyn MemoryCapability>, usize)| {
                let (dev, raddr) = dips;
                return dev.read_byte(raddr);
            });
        }
    }

impl MemoryWriteCapability for VirtualMemoryDevice {
    fn write_byte(&self, addr: usize, value: u8) -> bool {
        return self
        .dispatch_addr(addr)
        .and_then(|dips: (Rc<dyn MemoryCapability>, usize)| {
            let (dev, raddr) = dips;
            return Ok(dev.write_byte(raddr, value));
        })
        .unwrap_or(false);
}
}

impl MemoryCapability for VirtualMemoryDevice {}
*/

use crate::contexts::MemoryContext;

pub struct RegistersPage<T>
where
    T: Copy,
{
    pub registers: RefCell<Vec<T>>,
}

impl<T> RegistersPage<T>
where
    T: Copy,
{
    pub fn new(space: usize, default: T) -> RegistersPage<T> {
        RegistersPage::<T> {
            registers: RefCell::new(vec![default; space]),
        }
    }

    pub fn read(&self, addr: usize) -> T {
        self.registers.borrow()[addr]
    }

    pub fn write(&self, addr: usize, value: T) {
        self.registers.borrow_mut()[addr] = value;
    }
}

impl<T> Context for RegistersPage<T> where T: Copy {}

impl<T> MemoryContext for RegistersPage<T>
where
    T: Copy,
{
    type Output = T;

    fn read_bytes(&self, addr: usize, count: usize) -> Vec<Self::Output> {
        self.registers.borrow()[addr..addr + count].to_vec()
    }

    fn write_bytes(&self, addr: usize, bites: Vec<Self::Output>) {
        let mut mem = self.registers.borrow_mut();
        let len = mem.len();
        // let mut bites_iter: std::slice::Iter<T> = bites.iter();
        let mut ptr = addr;

        for byte in bites {
            if ptr < len {
                mem[ptr] = byte;
            }
            ptr += 1;
        }
    }
}

#[derive(Debug)]
pub struct MemoryDevice<T> {
    pub capacity: usize,
    data: RefCell<Vec<T>>,
}

impl<T> MemoryDevice<T>
where
    T: Default + Copy,
{
    pub fn new(size: usize) -> MemoryDevice<T> {
        MemoryDevice {
            capacity: size,
            data: RefCell::new(vec![T::default(); size]),
        }
    }

    pub fn from(data: Vec<T>) -> MemoryDevice<T> {
        MemoryDevice {
            capacity: data.len(),
            data: RefCell::new(data),
        }
    }
}

impl<T> Context for MemoryDevice<T> where T: Copy {}

impl<T> MemoryContext for MemoryDevice<T>
where
    T: Copy,
{
    type Output = T;

    fn read_bytes(&self, addr: usize, count: usize) -> Vec<Self::Output> {
        self.data.borrow()[addr..addr + count].to_vec()
    }

    fn write_bytes(&self, addr: usize, bites: Vec<Self::Output>) {
        let mut mem = self.data.borrow_mut();
        let len = mem.len();
        // let mut bites_iter = bites.iter();
        let mut ptr = addr;

        for byte in bites {
            if ptr < len {
                mem[ptr] = byte;
            }
            ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::contexts::MemoryContext;

    use super::RegistersPage;

    #[test]
    fn register_page() {
        let rp = RegistersPage::new(32, 0u8);

        assert_eq!(rp.read(12), 0);
        rp.write(12, 0xf2);

        assert_eq!(rp.read_bytes(10, 2), vec![0u8; 2]);
        rp.write_bytes(10, vec![5, 7]);
        assert_eq!(rp.read_bytes(10, 2), vec!(5, 7));
    }
}
