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

pub struct LocalAssigment<C> {
    key: String,
    value: Node<C, DataWordSized>,
}

impl<'node, C> LocalAssigment<C> {
    pub fn new(key: &'node str, value: Node<C, DataWordSized>) -> Self {
        LocalAssigment {
            key: key.to_string(),
            value,
        }
    }
}

impl<T, C> AbstactSyntaxNode for LocalAssigment<C>
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

impl<C> BoxPorting for LocalAssigment<C> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct LocalAccess<T, C> {
    key: String,
    _pth: (PhantomData<T>, PhantomData<C>),
}

impl<T, C> LocalAccess<T, C> {
    pub fn new(key: &str) -> Self {
        LocalAccess {
            key: key.to_string(),
            _pth: (PhantomData, PhantomData),
        }
    }
}

impl<C, T> AbstactSyntaxNode for LocalAccess<T, C>
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
