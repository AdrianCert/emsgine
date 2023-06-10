use std::sync::Arc;

use emsgine_lib::{
    ast::{AbstactSyntaxNode, Node},
    contexts::locals::{LocalEnviroment, SCOPES},
    lookup::Lookup,
    models::{
        bytes::DataWordSized,
        instructionset::{AstInstruction, FormatInstruction},
    },
};

pub struct CpuInstruction<C, T> {
    pub name: String,
    pub local: Arc<LocalEnviroment<DataWordSized>>,
    pub ast: Node<C, T>,
}

impl<'node, C, T> CpuInstruction<C, T> {
    pub fn from_instruction<InstructionSet, InstructionArg>(
        inst_id: InstructionSet,
        inst_arg: &InstructionArg,
    ) -> Self
    where
        InstructionSet: AstInstruction<Context = C, Output = T> + FormatInstruction,
        InstructionArg: Lookup<&'node str, DataWordSized>,
    {
        CpuInstruction {
            name: inst_id.format(inst_arg),
            local: Arc::new(inst_arg.into()),
            ast: inst_id.get_ast(),
        }
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
