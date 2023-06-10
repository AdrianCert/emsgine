// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=binary%20-%20op
// ██████╗ ██╗███╗   ██╗ █████╗ ██████╗ ██╗   ██╗               ██████╗ ██████╗
// ██╔══██╗██║████╗  ██║██╔══██╗██╔══██╗╚██╗ ██╔╝              ██╔═══██╗██╔══██╗
// ██████╔╝██║██╔██╗ ██║███████║██████╔╝ ╚████╔╝     █████╗    ██║   ██║██████╔╝
// ██╔══██╗██║██║╚██╗██║██╔══██║██╔══██╗  ╚██╔╝      ╚════╝    ██║   ██║██╔═══╝
// ██████╔╝██║██║ ╚████║██║  ██║██║  ██║   ██║                 ╚██████╔╝██║
// ╚═════╝ ╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝                  ╚═════╝ ╚═╝

use crate::models::portable::BoxPorting;

use super::AbstactSyntaxNode;
use super::Node;
use super::PrimitiveOperation;

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

pub struct BinaryOperationNode<C, T> {
    opr: AstBinaryOperation,
    rhs: Node<C, T>,
    lhs: Node<C, T>,
}

impl<C, T> BinaryOperationNode<C, T> {
    pub fn new_and(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::LogicalAnd,
            rhs,
            lhs,
        }
    }

    pub fn new_or(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::LogicalOr,
            rhs,
            lhs,
        }
    }

    pub fn new_xor(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::LogicalXor,
            rhs,
            lhs,
        }
    }

    pub fn new_add(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::ArithmeticAddition,
            rhs,
            lhs,
        }
    }

    pub fn new_sub(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::ArithmeticSubtraction,
            rhs,
            lhs,
        }
    }

    pub fn new_mul(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::ArithmeticMultiplication,
            rhs,
            lhs,
        }
    }

    pub fn new_div(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::ArithmeticDivision,
            rhs,
            lhs,
        }
    }

    pub fn new_sel(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::BitwiseSelect,
            rhs,
            lhs,
        }
    }

    pub fn new_shr(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::BitwiseShiftRight,
            rhs,
            lhs,
        }
    }

    pub fn new_shl(lhs: Node<C, T>, rhs: Node<C, T>) -> Self {
        BinaryOperationNode {
            opr: AstBinaryOperation::BitwiseShiftLeft,
            rhs,
            lhs,
        }
    }
}

impl<C, T> AbstactSyntaxNode for BinaryOperationNode<C, T>
where
    T: PrimitiveOperation<T>,
{
    type Output = T;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let lhs = self.lhs.eval(context);
        let rhs = self.rhs.eval(context);
        match self.opr {
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
}

impl<C, T> BoxPorting for BinaryOperationNode<C, T> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
