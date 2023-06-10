use std::marker::PhantomData;

use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::models::portable::BoxPorting;

use super::CpuContext;
use crate::ast::AbstactSyntaxNode;
use crate::ast::Node;

pub struct RegisterAccess<C>
where
    C: CpuContext,
{
    pub register: usize,
    _pth: PhantomData<C>,
}

impl<C> RegisterAccess<C>
where
    C: CpuContext,
{
    pub fn new(key: usize) -> Self {
        RegisterAccess {
            register: key,
            _pth: PhantomData,
        }
    }
}

impl<C> AbstactSyntaxNode for RegisterAccess<C>
where
    C: CpuContext,
{
    type Output = <Self::Context as CpuContext>::Register;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        context.register_get(self.register)
    }
}

impl<C> BoxPorting for RegisterAccess<C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct RegisterIndirectAccess<C>
where
    C: CpuContext,
{
    pub register: Node<C, DataWordSized>,
}

impl<C> RegisterIndirectAccess<C>
where
    C: CpuContext,
{
    pub fn new(key: Node<C, DataWordSized>) -> Self {
        RegisterIndirectAccess { register: key }
    }
}

impl<C> AbstactSyntaxNode for RegisterIndirectAccess<C>
where
    C: CpuContext,
{
    type Output = <Self::Context as CpuContext>::Register;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let reg = self.register.eval(context);
        context.register_get(reg.as_u64() as usize)
    }
}

impl<C> BoxPorting for RegisterIndirectAccess<C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct RegisterAssign<C>
where
    C: CpuContext,
{
    pub register: usize,
    pub value: Node<C, C::Register>,
}

impl<C> RegisterAssign<C>
where
    C: CpuContext,
{
    pub fn new(register: usize, value: Node<C, C::Register>) -> Self {
        RegisterAssign { register, value }
    }
}

impl<C> AbstactSyntaxNode for RegisterAssign<C>
where
    C: CpuContext,
{
    type Output = ();
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let value = self.value.eval(context);
        context.register_set(self.register, value);
    }
}

impl<C> BoxPorting for RegisterAssign<C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct RegisterIndirectAssign<C>
where
    C: CpuContext,
{
    pub register: Node<C, DataWordSized>,
    pub value: Node<C, C::Register>,
}

impl<C> RegisterIndirectAssign<C>
where
    C: CpuContext,
{
    pub fn new(register: Node<C, DataWordSized>, value: Node<C, C::Register>) -> Self {
        RegisterIndirectAssign { register, value }
    }
}

impl<C> AbstactSyntaxNode for RegisterIndirectAssign<C>
where
    C: CpuContext,
{
    type Output = ();
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let value = self.value.eval(context);
        let register = self.register.eval(context);
        context.register_set(register.as_u64() as usize, value);
    }
}

impl<C> BoxPorting for RegisterIndirectAssign<C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
