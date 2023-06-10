// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=unary%20-%20op
// ██╗   ██╗███╗   ██╗ █████╗ ██████╗ ██╗   ██╗               ██████╗ ██████╗
// ██║   ██║████╗  ██║██╔══██╗██╔══██╗╚██╗ ██╔╝              ██╔═══██╗██╔══██╗
// ██║   ██║██╔██╗ ██║███████║██████╔╝ ╚████╔╝     █████╗    ██║   ██║██████╔╝
// ██║   ██║██║╚██╗██║██╔══██║██╔══██╗  ╚██╔╝      ╚════╝    ██║   ██║██╔═══╝
// ╚██████╔╝██║ ╚████║██║  ██║██║  ██║   ██║                 ╚██████╔╝██║
//  ╚═════╝ ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝                  ╚═════╝ ╚═╝

use crate::models::portable::BoxPorting;

use super::AbstactSyntaxNode;
use super::Node;
use super::PrimitiveOperation;

pub enum AstUnaryOperation {
    LogicalNegation,
    ArithmeticIncrement,
    ArithmeticDecrement,
}

pub struct UnaryOperationNode<C, T> {
    opr: AstUnaryOperation,
    child: Node<C, T>,
}

impl<C, T> UnaryOperationNode<C, T> {
    pub fn new_neg(child: Node<C, T>) -> Self {
        UnaryOperationNode {
            opr: AstUnaryOperation::LogicalNegation,
            child,
        }
    }

    pub fn new_inc(child: Node<C, T>) -> Self {
        UnaryOperationNode {
            opr: AstUnaryOperation::ArithmeticIncrement,
            child,
        }
    }
    pub fn new_dec(child: Node<C, T>) -> Self {
        UnaryOperationNode {
            opr: AstUnaryOperation::ArithmeticDecrement,
            child,
        }
    }
}

impl<C, T> AbstactSyntaxNode for UnaryOperationNode<C, T>
where
    T: PrimitiveOperation<T>,
{
    type Output = T;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        let child = self.child.eval(context);
        match self.opr {
            AstUnaryOperation::LogicalNegation => !child,
            AstUnaryOperation::ArithmeticIncrement => child.increment(),
            AstUnaryOperation::ArithmeticDecrement => child.decrement(),
        }
    }
}

impl<C, T> BoxPorting for UnaryOperationNode<C, T> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
