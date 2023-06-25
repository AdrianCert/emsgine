use emsgine_lib::{ast::Node, models::instructionset::InstructionNamespace};

use crate::contexts::CpuContext;

pub trait AstFactoryProvider<ISET, Context>
where
    ISET: InstructionNamespace,
    Context: CpuContext,
{
    type Output;

    fn get_ast(&self, instruction: &ISET) -> Option<Node<Context, Self::Output>>;
}
