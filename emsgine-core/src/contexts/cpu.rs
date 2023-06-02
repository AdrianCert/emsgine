use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::contexts::Context;
use crate::contexts::memory::PointerContext;


pub trait CpuContext: Context {
    type Register;
    type Pointer: PointerContext;

    fn register_get(&self, register: usize) -> Self::Register;
    fn register_set(&mut self, register: usize, value: Self::Register);
    fn register_update(&mut self, register: usize, update: fn(Self::Register) -> Self::Register);
    fn memory_pull(&self, addr: Self::Pointer, count: usize) -> Vec<u8>;
    fn memory_push(&mut self, addr: Self::Pointer, value: Vec<u8>);
    fn states_get(&self, key: &str) -> DataWordSized;
    fn states_set(&mut self, key: &str, value: DataWordSized);
    fn states_update(&mut self, key: &str, update: fn(DataWordSized) -> DataWordSized);
}
