use std::marker::PhantomData;

use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::models::portable::BoxPorting;

use super::CpuContext;
use crate::ast::AbstactSyntaxNode;
use crate::ast::Node;

pub struct SymbolAccess<'node, C>
where
    C: CpuContext,
{
    pub symbol: &'node str,
    _pth: PhantomData<C>,
}

impl<'node, C> SymbolAccess<'node, C>
where
    C: CpuContext,
{
    pub fn new(symbol: &'node str) -> Self {
        SymbolAccess {
            symbol,
            _pth: PhantomData,
        }
    }
}

impl<'node, C> AbstactSyntaxNode for SymbolAccess<'node, C>
where
    C: CpuContext,
{
    type Output = DataWordSized;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        context.states_get(self.symbol)
    }
}

impl<'node, C> BoxPorting for SymbolAccess<'node, C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct SymbolIndirectAccess<C>
where
    C: CpuContext,
{
    pub symbol: Node<C, String>,
}

impl<C> SymbolIndirectAccess<C>
where
    C: CpuContext,
{
    pub fn new(symbol: Node<C, String>) -> Self {
        SymbolIndirectAccess { symbol }
    }
}

impl<C> AbstactSyntaxNode for SymbolIndirectAccess<C>
where
    C: CpuContext,
{
    type Output = DataWordSized;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let symbol = self.symbol.eval(context);
        context.states_get(symbol.as_str())
    }
}

impl<C> BoxPorting for SymbolIndirectAccess<C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct SymbolAssign<'node, C>
where
    C: CpuContext,
{
    pub symbol: &'node str,
    pub value: Node<C, DataWordSized>,
}

impl<'node, C> SymbolAssign<'node, C>
where
    C: CpuContext,
{
    pub fn new(symbol: &'node str, value: Node<C, DataWordSized>) -> Self {
        SymbolAssign { symbol, value }
    }
}

impl<'node, C> AbstactSyntaxNode for SymbolAssign<'node, C>
where
    C: CpuContext,
{
    type Output = ();
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let value = self.value.eval(context);
        context.states_set(self.symbol, value);
    }
}

impl<'node, C> BoxPorting for SymbolAssign<'node, C>
where
    C: CpuContext,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
