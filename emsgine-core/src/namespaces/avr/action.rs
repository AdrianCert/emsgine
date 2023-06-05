// use crate::{actions::{
//     // InstructionPull,
//     // InstructionBuild,
//     // RegisterAccess,
//     // RegisterAssign
// }, contexts::CpuState};

// use crate::contexts::CpuContext;

// use crate::actions::registers::{RegisterAccess, RegisterAssign};
// use crate::contexts::CpuContext;
// use crate::actions::instruction::CpuInstruction;

// use emsgine_lib::{actions::{
//     // ActionEvaluator,
//     Operation,
//     Action
// }, ast::AstTree, ast::AstEvaluator, ast::AstNode, ast::AstBinaryOperation, ast::AstContextAction};

// pub trait InstructionBuild{
//     type Register;

//     build
// }

// pub fn adc(rd: u8, rs: u8) ->

// pub fn build_add(rd: u8, rs: u8) -> CpuInstruction {
//     return CpuInstruction {
//         actions: vec![
//             RegisterAssign::<u8> {
//                 register: rd,
//                 value: AstTree {
//                     evaluator: AstEvaluator::<u8> {
//                         parameters: Box::new(vec![])
//                     },
//                     node: AstNode::BinaryOperation {
//                         opr: &AstBinaryOperation::ArithmeticAddition,
//                         rhs: AstContextAction::<CpuContext, > RegisterAccess {
//                             register: rs
//                         },
//                         lhs: RegisterAccess {
//                             register: rd
//                         }
//                     }
//                 }
//             }
//         ]
//     };
//     return RegisterAssign::new(rd, value)
//     return RegisterAssign::<u8> {
//         register: rd,
//         value: AstTree {
//             operand_1: RegisterAccess {
//                 register: rd
//             },
//             operand_2: RegisterAccess {
//                 register: rs
//             },
//             operation: |a, b| { a + b }
//         }
//     }
// }

// pub fn build_adc<T>(rd: u8, rs: u8) -> dyn Action::<T> {
//     return RegisterAssign::<u8> {
//         register: rd,
//         value: Operation {
//             operand_1: RegisterAccess {
//                 register: rd
//             },
//             operand_2: RegisterAccess {
//                 register: rs
//             },
//             operation: |a, b| { a + b }
//         }
//     }
// }
