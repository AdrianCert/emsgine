// use std::rc::Rc;

// use emsgine_lib::actions::Action;
// use crate::contexts::CpuContext;

// use crate::cpu::CpuContext;

pub struct CpuConfig<'a, T> {
    pub parameters: Box<Vec<(&'a str, T)>>,
}

pub struct CpuInstruction<'a, R> {
    // pub actions: Vec<Rc<dyn Action<dyn CpuContext<Register = R>>>>,
    pub name: String,
    pub config: CpuConfig<'a, R>,
}
