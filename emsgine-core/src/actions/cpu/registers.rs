pub use emsgine_lib::models::instructionset::InstructionNamespace;
use emsgine_lib::primitive::ContextValue;
use emsgine_lib::{
    actions::{Action, ActionEvaluator},
    contexts::ContextParameter,
};

use crate::contexts::CpuContext;

// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=ReG%20-%20Assign
// ██████╗ ███████╗ ██████╗                █████╗ ███████╗███████╗██╗ ██████╗ ███╗   ██╗
// ██╔══██╗██╔════╝██╔════╝               ██╔══██╗██╔════╝██╔════╝██║██╔════╝ ████╗  ██║
// ██████╔╝█████╗  ██║  ███╗    █████╗    ███████║███████╗███████╗██║██║  ███╗██╔██╗ ██║
// ██╔══██╗██╔══╝  ██║   ██║    ╚════╝    ██╔══██║╚════██║╚════██║██║██║   ██║██║╚██╗██║
// ██║  ██║███████╗╚██████╔╝              ██║  ██║███████║███████║██║╚██████╔╝██║ ╚████║
// ╚═╝  ╚═╝╚══════╝ ╚═════╝               ╚═╝  ╚═╝╚══════╝╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═══╝

pub struct RegisterAssign<'a, C>
where
    C: CpuContext,
{
    pub register: ContextValue<'a, usize>,
    pub value: &'a dyn ActionEvaluator<C, C::Register>,
}

impl<C, R, P> RegisterAssign<'_, C>
where
    C: CpuContext<Register = R, Pointer = P>,
{
    pub fn new<'a>(
        register: ContextValue<'a, usize>,
        value: &'a dyn ActionEvaluator<C, C::Register>,
    ) -> RegisterAssign<'a, C> {
        RegisterAssign { register, value }
    }

    pub fn new_register(
        register: usize,
        value: &dyn ActionEvaluator<C, C::Register>,
    ) -> RegisterAssign<'_, C> {
        RegisterAssign {
            register: ContextValue::Immediate(register),
            value,
        }
    }
}

impl<C> Action<C> for RegisterAssign<'_, C>
where
    C: CpuContext + ContextParameter<Output = usize>,
{
    fn run(&self, context: &mut C) {
        let val = self.value.eval(context);
        context.register_set(self.register.resolve(context).unwrap(), val);
    }
}

pub struct RegisterIndirectAssign<'ast, C>
where
    C: CpuContext,
{
    pub register: &'ast dyn ActionEvaluator<C, usize>,
    pub value: dyn ActionEvaluator<C, C::Register>,
}

// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=ReG%20-%20Access
// ██████╗ ███████╗ ██████╗                █████╗  ██████╗ ██████╗███████╗███████╗███████╗
// ██╔══██╗██╔════╝██╔════╝               ██╔══██╗██╔════╝██╔════╝██╔════╝██╔════╝██╔════╝
// ██████╔╝█████╗  ██║  ███╗    █████╗    ███████║██║     ██║     █████╗  ███████╗███████╗
// ██╔══██╗██╔══╝  ██║   ██║    ╚════╝    ██╔══██║██║     ██║     ██╔══╝  ╚════██║╚════██║
// ██║  ██║███████╗╚██████╔╝              ██║  ██║╚██████╗╚██████╗███████╗███████║███████║
// ╚═╝  ╚═╝╚══════╝ ╚═════╝               ╚═╝  ╚═╝ ╚═════╝ ╚═════╝╚══════╝╚══════╝╚══════

pub struct RegisterAccess {
    pub register: usize,
}

impl RegisterAccess {
    pub fn new(register: usize) -> RegisterAccess {
        RegisterAccess { register }
    }
}

impl<C> ActionEvaluator<C, C::Register> for RegisterAccess
where
    C: CpuContext,
{
    fn eval(&self, context: &mut C) -> C::Register {
        context.register_get(self.register)
    }
}

pub struct IndirectRegisterAccess<C> {
    pub register: dyn ActionEvaluator<C, u8>,
}
