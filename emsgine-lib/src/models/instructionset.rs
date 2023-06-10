use super::bytes::DataWordSized;
use crate::{ast::Node, lookup::Lookup};

pub trait InstructionNamespace {}

pub trait MnemonicInstruction: InstructionNamespace {
    fn mnemonic<'a>(&self) -> &'a str;
}

pub trait FormatInstruction: InstructionNamespace {
    fn formatstr<'a>(&self) -> &'a str;
    fn format(&self, ltbl: &dyn Lookup<&str, DataWordSized>) -> String;
}

pub trait AstInstruction: InstructionNamespace {
    type Context;
    type Output;
    fn get_ast(&self) -> Node<Self::Context, Self::Output>;
}
