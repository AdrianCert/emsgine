use emsgine_lib::actions::ActionEvaluator;
use emsgine_lib::contexts::Context;
use emsgine_lib::models::bytes::EndianByteOrdering;
pub use emsgine_lib::models::instructionset::InstructionNamespace;

use crate::contexts::CpuContext;

pub mod instruction;
pub mod registers;

pub struct InstructionPull {
    pub wordsize: u8,
    pub endianness: EndianByteOrdering,
}


/// ChainOperation left to right lazy operations.
///
/// First send first two operands to first operation.
/// Next operand with previus result it's send to next operation.
/// If it's no next operation then it's used the last one.
pub struct ChainOperation<C, T>
where
    C: Context
{
    pub operands: Vec<Box<dyn ActionEvaluator<C, T>>>,
    pub operations: Vec<fn(T, T) -> T>,
}

use std::iter::repeat;

impl<C, T> ActionEvaluator<C, T> for ChainOperation<C, T>
where
    C: CpuContext,
    T: Copy
{
    fn eval(&self, context: &mut C) -> T {

        if self.operands.len() == 1 {
            return self.operands[0].eval(context);
        }

        let last_action = self.operations.last().unwrap();
        let mut iter_action = self.operations.iter().chain(repeat(last_action));
        let mut iter_opr = self.operands.iter();

        let mut prev_opr = iter_opr.next().unwrap().eval(context);

        iter_opr.for_each(|op| {
            let action = iter_action.next().unwrap();
            prev_opr = action(prev_opr, op.eval(context));
        });

        prev_opr
    }
}

pub struct LogicalAnd<C, T> {
    pub operands: Vec<Box<dyn ActionEvaluator<C, T>>>
}

pub struct LogicalOr<C, T> {
    pub operands: Vec<Box<dyn ActionEvaluator<C, T>>>
}

pub struct LogicalNegation<C, T> {
    pub value: Box<dyn ActionEvaluator<C, T>>
}

impl<C, T> LogicalNegation<C, T> {
    pub fn new(value: Box<dyn ActionEvaluator<C, T>>) -> LogicalNegation<C, T> {
        LogicalNegation {
            value
        }
    }
}

pub struct UnaryOperation<C, T>
where
    C: Context
 {
    pub operand: Box<dyn ActionEvaluator<C, T>>,
    pub operation: fn(T) -> T,
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{cell::RefCell};
//     use crate::memory::RegistersPage;

//     struct DummyCpuRegs {
//         register_page: RegistersPage,
//     }

//     #[test]
//     fn it_works() {
//         let chain_operation = ChainOperation::<DummyCpuRegs, u8> {
//             operands: vec![
//                 Box::new(RegisterAccess{
//                     register: 1
//                 }),
//                 Box::new(RegisterAccess{
//                     register: 2
//                 }),
//                 Box::new(RegisterAccess{
//                     register: 3
//                 }),
//                 Box::new(RegisterAccess{
//                     register: 4
//                 }),
//                 Box::new(LogicalNegation::<DummyCpuRegs, u8>::new(RegisterAccess::new(5))),
//                 Box::new(RegisterAccess::new(6)),
//                 Box::new(RegisterAccess::new(7)),
//             ],
//             operations: vec![
//                 |a, b| { a & b}
//             ]
//         };
//         let mut context = DummyCpuRegs {
//             register_page: RegistersPage::<u8> {
//                 registers: RefCell::new(vec![0xff; 10])
//             }
//         };
//         let value = chain_operation.eval(&mut context);
//     }
// }




impl<T, C> ActionEvaluator<C, T> for UnaryOperation<C, T>
where
    C: Context,
{
    fn eval(&self, context: &mut C) -> T {
        return (self.operation)(self.operand.eval(context));
    }
}

// impl<T, C> ActionEvaluator<T, C> for BinaryOperation<C, T> {
//     fn eval(&self, context: &mut C) -> T {
//         (self.operation)(self.operand_a.eval(context), self.operand_a.eval(context))
//     }
// }
