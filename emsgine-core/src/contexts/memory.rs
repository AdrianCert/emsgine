use std::rc::Rc;

use emsgine_lib::contexts::Context;

pub trait MemoryContext: Context {
    type Output;
    fn read_bytes(&self, addr: usize, count: usize) -> Vec<Self::Output>;
    fn write_bytes(&self, addr: usize, bites: Vec<Self::Output>);
}

pub trait PointerContext {
    type Context: Context;
    type Output;

    fn resolve(
        &self,
        context: &Self::Context,
    ) -> (usize, Rc<dyn MemoryContext<Output = Self::Output>>);
}
