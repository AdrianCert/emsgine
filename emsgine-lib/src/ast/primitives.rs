use crate::{
    contexts::{locals::SCOPES, Context, ContextParameter},
    models::{bytes::DataWordSized, portable::BoxPorting},
};

use super::{AbstactSyntaxNode, Node};
use std::marker::PhantomData;

pub struct Immediate<T, C>
where
    T: Copy,
{
    pub value: T,
    _pth: PhantomData<C>,
}

impl<T, C> AbstactSyntaxNode for Immediate<T, C>
where
    T: Copy,
{
    type Context = C;
    type Output = T;

    fn eval(&self, _context: &mut Self::Context) -> Self::Output {
        self.value
    }
}

impl<T, C> Immediate<T, C>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Immediate {
            value,
            _pth: PhantomData,
        }
    }
}

impl<T, C> BoxPorting for Immediate<T, C>
where
    T: Copy,
{
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

// pub struct Parameter<'node, T, C>
// where
//     C: Context + ContextParameter<Output = T>,
// {
//     pub param: &'node str,
//     _pth: (PhantomData<T>, PhantomData<C>),
// }

// impl<'node, T, C> AbstactSyntaxNode for Parameter<'node, T, C>
// where
//     C: Context + ContextParameter<Output = T>,
//     T: Copy,
// {
//     type Output = T;

//     type Context = C;

//     fn eval(&self, context: &mut Self::Context) -> Self::Output {
//         context.param(self.param).unwrap()
//     }
// }

pub struct LocalIndirectAssigment<C> {
    key: String,
    value: Node<C, DataWordSized>,
}

impl<'node, C> LocalIndirectAssigment<C> {
    pub fn new(key: &'node str, value: Node<C, DataWordSized>) -> Self {
        LocalIndirectAssigment {
            key: key.to_string(),
            value,
        }
    }
}

impl<T, C> AbstactSyntaxNode for LocalIndirectAssigment<C>
where
    C: Context + ContextParameter<Output = T>,
    T: Copy,
{
    type Context = C;
    type Output = ();

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let result = self.value.eval(context);
        SCOPES.variable_set(&self.key, result);
    }
}

impl<C> BoxPorting for LocalIndirectAssigment<C> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct LocalAccess<C, T> {
    key: String,
    _pth: (PhantomData<T>, PhantomData<C>),
}

impl<C, T> LocalAccess<C, T> {
    pub fn new(key: &str) -> Self {
        LocalAccess {
            key: key.to_string(),
            _pth: (PhantomData, PhantomData),
        }
    }
}

impl<C, T> AbstactSyntaxNode for LocalAccess<C, T>
where
    C: Context,
    T: Copy,
{
    type Context = C;
    type Output = DataWordSized;

    fn eval(&self, _context: &mut Self::Context) -> Self::Output {
        SCOPES.variable_get(&self.key).unwrap()
    }
}

impl<T, C> BoxPorting for LocalAccess<T, C> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct NoOperation<C, T> {
    _pth: (PhantomData<T>, PhantomData<C>),
    _def: T,
}

impl<C, T> NoOperation<C, T> {
    pub fn new(default: T) -> Self {
        NoOperation {
            _pth: (PhantomData, PhantomData),
            _def: default,
        }
    }
}

impl<C, T> Default for NoOperation<C, T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<C, T> AbstactSyntaxNode for NoOperation<C, T>
where
    C: Context,
    T: Copy,
{
    type Context = C;
    type Output = T;

    fn eval(&self, _context: &mut Self::Context) -> Self::Output {
        self._def
    }
}

impl<T, C> BoxPorting for NoOperation<T, C> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
