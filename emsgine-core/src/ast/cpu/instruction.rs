use std::{collections::LinkedList, sync::Arc};

use emsgine_lib::{
    ast::{AbstactSyntaxNode, Node},
    contexts::locals::{LocalEnvironment, SCOPES},
    lookup::Lookup,
    models::{
        bytes::DataWordSized,
        instructionset::{FormatInstruction, InstructionNamespace},
    },
};

use crate::ast::provider::AstFactoryProvider;

use super::CpuContext;

pub struct CpuInstruction<C, T> {
    pub name: String,
    pub local: Arc<LocalEnvironment<DataWordSized>>,
    pub ast: Node<C, T>,
}

pub struct CpuInstructionFactory<ISET, C, T>
where
    ISET: InstructionNamespace + FormatInstruction,
{
    pub providers: LinkedList<Box<dyn AstFactoryProvider<ISET, C, Output = T>>>,
}

impl<'factory, ISET, C, T> CpuInstructionFactory<ISET, C, T>
where
    ISET: InstructionNamespace + FormatInstruction,
    C: CpuContext,
    T: Copy,
{
    pub fn parse<ARGS>(&self, inst: &ISET, param: &ARGS) -> Option<CpuInstruction<C, T>>
    where
        ARGS: Lookup<&'factory str, DataWordSized>,
    {
        for provider in self.providers.iter() {
            if let Some(ast) = provider.get_ast(inst) {
                return Some(CpuInstruction {
                    name: inst.format(param),
                    local: Arc::new(param.into()),
                    ast,
                });
            }
        }

        None
    }
}

impl<C, T> AbstactSyntaxNode for CpuInstruction<C, T> {
    type Output = T;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        SCOPES.push_local(self.local.clone());
        let result = self.ast.eval(context);
        SCOPES.drop_local();
        result
    }
}
