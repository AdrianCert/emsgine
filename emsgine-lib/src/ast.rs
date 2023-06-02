use crate::actions::ActionEvaluator;
use crate::contexts::Context;
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};
use crate::contexts::ContextParameter;

use crate::bitwise::{BitSel, Decrement, Increment};

pub trait Interpreter<C>
where
    C: Context,
{
    type Output;

    fn eval(&self, node: &AstNode<C, Self::Output>, context: &mut C) -> Self::Output;
}

pub enum AstUnaryOperation {
    LogicalNegation,
    ArithmeticIncrement,
    ArithmeticDecrement,
}

pub enum AstBinaryOperation {
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    ArithmeticAddition,
    ArithmeticSubtraction,
    ArithmeticMultiplication,
    ArithmeticDivision,
    BitwiseSelect,
    BitwiseShiftLeft,
    BitwiseShiftRight,
}

pub struct AstContextAction<C, T> {
    pub action: Box<dyn ActionEvaluator<C, T>>,
}

pub enum AstNode<'a, C, T> {
    Immediate(T),
    Parameter(String),
    BinaryOperation {
        opr: AstBinaryOperation,
        rhs: &'a AstNode<'a, C, T>,
        lhs: &'a AstNode<'a, C, T>,
    },
    UnaryOperation {
        opr: AstUnaryOperation,
        child: &'a AstNode<'a, C, T>,
    },
    ContextAction(Box<dyn ActionEvaluator<C, T>>),
}

pub struct GenericAstTree<'a, T, C> {
    pub params: Box<Vec<(&'a str, T)>>,
    pub root: AstNode<'a, C, T>,
}

/// To be tested if work as a shorter
trait PrimitiveOperation<T>: BitAnd<T, Output = T>
+ BitOr<T, Output = T>
+ BitXor<T, Output = T>
+ Not<Output = T>
+ Add<T, Output = T>
+ Sub<T, Output = T>
+ Mul<T, Output = T>
+ Div<T, Output = T>
+ Shl<T, Output = T>
+ Shr<T, Output = T>
+ BitSel<T, Output = T>
+ Increment<T, Output = T>
+ Decrement<T, Output = T> {}

impl<'a, T, C> ActionEvaluator<C, T> for AstNode<'a, C, T>
where
    C: Context + ContextParameter<Output=T>,
    T: Copy
        + Default
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>
        + BitSel<Output = T>
        + Increment<Output = T>
        + Decrement<Output = T>,
{
    fn eval(&self, context: &mut C) -> T {
        match self {
            AstNode::Immediate(value) => value.clone(),
            AstNode::Parameter(value) => context.param(value).unwrap_or_default(),
            AstNode::BinaryOperation { opr, rhs, lhs } => {
                let lhs = lhs.eval(context);
                let rhs = rhs.eval(context);
                match opr {
                    AstBinaryOperation::LogicalAnd => lhs & rhs,
                    AstBinaryOperation::LogicalOr => lhs | rhs,
                    AstBinaryOperation::LogicalXor => lhs ^ rhs,
                    AstBinaryOperation::ArithmeticAddition => lhs + rhs,
                    AstBinaryOperation::ArithmeticSubtraction => lhs - rhs,
                    AstBinaryOperation::ArithmeticMultiplication => lhs * rhs,
                    AstBinaryOperation::ArithmeticDivision => lhs / rhs,
                    AstBinaryOperation::BitwiseSelect => lhs.bitsel(rhs),
                    AstBinaryOperation::BitwiseShiftLeft => lhs >> rhs,
                    AstBinaryOperation::BitwiseShiftRight => lhs << rhs,
                }
            }
            AstNode::UnaryOperation { opr, child } => {
                let child = child.eval(context);
                match opr {
                    AstUnaryOperation::LogicalNegation => !child,
                    AstUnaryOperation::ArithmeticIncrement => child.increment(),
                    AstUnaryOperation::ArithmeticDecrement => child.decrement(),
                }
            }
            AstNode::ContextAction(action) => action.eval(context),
        }
    }
}

pub struct AstTree<'a, E, C, T>
where
    E: Interpreter<C, Output = T>,
    C: Context,
{
    pub evaluator: E,
    pub node: &'a AstNode<'a, C, T>,
}

impl<'a, E, C, T> ActionEvaluator<C, T> for AstTree<'a, E, C, T>
where
    C: Context,
    E: Interpreter<C, Output = T>,
    T: Copy
{
    fn eval(&self, context: &mut C) -> T {
        self.evaluator.eval(self.node, context)
    }
}

impl<'a, E, C, T> AstTree<'a, E, C, T>
where
    C: Context,
    E: Interpreter<C, Output = T>,
{
    // pub fn new(params: Vec<(&str, T)>) -> AstTree<'a, E, C, T> {
    //     // return AstTree::<'a, E, C, T> {
    //     //     evaluator: ASt
    //     // };
    //     // parameters: Box::new(params),
    // }
}

#[derive(Debug)]
pub struct AstEvaluator<'a, T> {
    pub parameters: Box<Vec<(&'a str, T)>>,
}

impl<'a, T> AstEvaluator<'a, T>
where
    T: Copy,
{
    pub fn new(params: Vec<(&str, T)>) -> AstEvaluator<T> {
        return AstEvaluator::<T> {
            parameters: Box::new(params),
        };
    }

    pub fn param(&self, value: &str) -> Option<T> {
        let items = self.parameters.as_ref();
        for (key, param) in items {
            if value.cmp(key) == core::cmp::Ordering::Equal {
                return Some(param.clone());
            }
        }
        None
    }
}

impl<'evaluator, 'ast, T, C> Interpreter<C> for AstEvaluator<'evaluator, T>
where
    C: Context,
    T: Copy
        + Default
        + From<u8>
        + BitAnd<Output = T>
        + BitOr<Output = T>
        + BitXor<Output = T>
        + Not<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Shl<Output = T>
        + Shr<Output = T>
        + BitSel<Output = T>
        + Increment<Output = T>
        + Decrement<Output = T>,
{
    type Output = T;

    fn eval(&self, node: &AstNode<C, Self::Output>, context: &mut C) -> Self::Output {
        match node {
            AstNode::Immediate(value) => value.clone(),
            AstNode::Parameter(value) => self.param(value).unwrap_or_default(),
            AstNode::BinaryOperation { opr, rhs, lhs } => {
                let lhs = self.eval(lhs, context);
                let rhs = self.eval(rhs, context);
                match opr {
                    AstBinaryOperation::LogicalAnd => lhs & rhs,
                    AstBinaryOperation::LogicalOr => lhs | rhs,
                    AstBinaryOperation::LogicalXor => lhs ^ rhs,
                    AstBinaryOperation::ArithmeticAddition => lhs + rhs,
                    AstBinaryOperation::ArithmeticSubtraction => lhs - rhs,
                    AstBinaryOperation::ArithmeticMultiplication => lhs * rhs,
                    AstBinaryOperation::ArithmeticDivision => lhs / rhs,
                    AstBinaryOperation::BitwiseSelect => lhs.bitsel(rhs),
                    AstBinaryOperation::BitwiseShiftLeft => lhs >> rhs,
                    AstBinaryOperation::BitwiseShiftRight => lhs << rhs,
                }
            }
            AstNode::UnaryOperation { opr, child } => {
                let child = self.eval(child, context);
                match opr {
                    AstUnaryOperation::LogicalNegation => !child,
                    AstUnaryOperation::ArithmeticIncrement => child.increment(),
                    AstUnaryOperation::ArithmeticDecrement => child.decrement(),
                }
            }
            AstNode::ContextAction(action) => action.eval(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AstEvaluator;

    #[test]
    fn allow_reuse() {
        let ev = AstEvaluator::new(vec![("d", 1), ("3", 2)]);

        assert_eq!(ev.param("d"), Some(1));
        assert_eq!(ev.param("d"), Some(1));
        assert_eq!(ev.param("3"), Some(2));
    }
}
