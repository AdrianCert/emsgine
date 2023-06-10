use crate::contexts::CpuContext;
use crate::contexts::MemoryContext;
use crate::contexts::PointerContext;
use crate::memory::MemoryDevice;
use emsgine_lib::contexts::Context;
use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::models::bytes::EndianByteOrdering;
use std::collections::BTreeMap;
use std::rc::Rc;

pub static ENDIANNESS: EndianByteOrdering = EndianByteOrdering::LittleEndian;

#[derive(Debug)]
pub struct MemoryMap {
    pub reg_page: usize,
    pub io_space: usize,
    pub data_space: usize,
    pub overflow: usize,
}

#[derive(Debug)]
pub struct SymbolTable {
    vtbl: BTreeMap<&'static str, (AddressPointer, u8)>,
}

impl SymbolTable {
    pub fn create(value: Vec<(&'static str, (AddressPointer, u8))>) -> SymbolTable {
        let mut vtbl = BTreeMap::new();
        vtbl.extend(value);
        SymbolTable { vtbl }
    }

    pub fn get(&self, key: &str, context: &CentralProcessUnit) -> DataWordSized {
        if let Some(syb) = self.vtbl.get(key) {
            let (ptr, size) = syb;
            let (radr, mem) = ptr.resolve(context);
            let bytes = mem.read_bytes(radr, usize::from(*size));
            return ENDIANNESS.compose(bytes);
        }
        DataWordSized::Invalid
    }

    pub fn set(&self, key: &str, context: &CentralProcessUnit, value: DataWordSized) {
        if let Some(syb) = self.vtbl.get(key) {
            let (ptr, _) = syb;
            let (radr, mem) = ptr.resolve(context);
            let bytes = ENDIANNESS.settle(value);
            mem.write_bytes(radr, bytes);
        }
    }
}

#[derive(Debug)]
pub enum AddressPointer {
    RegisterPage(usize), // first 32 registers value
    IoSpace(usize),      // io ports map here
    DataSpace(usize),    // rest of data here
    MemorySpace(usize),  // used to absolute maps
    ProgramSpace(usize), // used to store program
    SpecialSpace(usize),
}

#[derive(Debug)]
pub struct CentralProcessUnit {
    pub mram: Rc<MemoryDevice<u8>>,
    pub mprg: Rc<MemoryDevice<u8>>,
    pub mspc: Rc<MemoryDevice<u8>>,
    pub mmap: MemoryMap,
    pub stbl: SymbolTable,
}

impl AddressPointer {
    fn read_bytes(&self, context: &CentralProcessUnit, count: usize) -> Vec<u8> {
        let (radr, ctx) = self.resolve(context);
        ctx.read_bytes(radr, count)
    }

    fn write_bytes(&self, context: &CentralProcessUnit, bytes: Vec<u8>) {
        let (radr, ctx) = self.resolve(context);
        ctx.write_bytes(radr, bytes)
    }

    fn read(&self, context: &CentralProcessUnit) -> u8 {
        self.read_bytes(context, 1)[0]
    }

    fn write(&self, context: &CentralProcessUnit, value: u8) {
        self.write_bytes(context, vec![value])
    }
}

impl PointerContext for AddressPointer {
    type Context = CentralProcessUnit;
    type Output = u8;

    fn resolve(
        &self,
        context: &Self::Context,
    ) -> (usize, std::rc::Rc<dyn MemoryContext<Output = Self::Output>>) {
        match self {
            AddressPointer::RegisterPage(val) => {
                (context.mmap.reg_page + val, context.mram.clone())
            }
            AddressPointer::IoSpace(val) => (context.mmap.io_space + val, context.mram.clone()),
            AddressPointer::DataSpace(val) => (context.mmap.data_space + val, context.mram.clone()),
            AddressPointer::MemorySpace(val) => (*val, context.mram.clone()),
            AddressPointer::ProgramSpace(val) => (*val, context.mprg.clone()),
            AddressPointer::SpecialSpace(val) => (*val, context.mspc.clone()),
        }
    }
}

impl Context for CentralProcessUnit {}
impl CpuContext for CentralProcessUnit {
    type Register = u8;
    type Pointer = AddressPointer;

    fn register_get(&self, register: usize) -> Self::Register {
        AddressPointer::RegisterPage(register).read(self)
    }

    fn register_set(&mut self, register: usize, value: Self::Register) {
        AddressPointer::RegisterPage(register).write(self, value)
    }

    fn register_update(&mut self, register: usize, update: fn(Self::Register) -> Self::Register) {
        let rprt = AddressPointer::RegisterPage(register);
        let rval = update(rprt.read(self));
        rprt.write(self, rval);
    }

    fn memory_pull(&self, addr: Self::Pointer, count: usize) -> Vec<u8> {
        addr.read_bytes(self, count)
    }

    fn memory_push(&mut self, addr: Self::Pointer, value: Vec<u8>) {
        addr.write_bytes(self, value)
    }

    fn states_get(&self, key: &str) -> DataWordSized {
        self.stbl.get(key, self)
    }

    fn states_set(&mut self, key: &str, value: DataWordSized) {
        self.stbl.set(key, self, value)
    }

    fn states_update(&mut self, key: &str, update: fn(DataWordSized) -> DataWordSized) {
        let cval = self.stbl.get(key, self);
        let nval = update(cval);
        self.stbl.set(key, self, nval);
    }
}
