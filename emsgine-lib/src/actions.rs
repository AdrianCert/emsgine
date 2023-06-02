use std::rc::Rc;

use crate::contexts::Context;


pub trait Action<C>
where
    C: Context,
{
    fn run(&self, context: &mut C);
}

pub struct Efect<C>
where
    C: Context,
{
    pub actions: Vec<Rc<dyn Action<C>>>,
}

pub trait ActionEvaluator<C, T>
where
    C: Context
{
    fn eval(&self, context: &mut C) -> T;
}
