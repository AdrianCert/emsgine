mod op_binary;
mod op_unary;
mod primitives;
use std::sync::atomic::{AtomicU64, Ordering};
mod statements;
use crate::bitwise::{BitSel, Decrement, Increment};
use core::ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Shl, Shr, Sub};

pub use op_binary::BinaryOperationNode;
pub use op_unary::UnaryOperationNode;
pub use primitives::Immediate;
pub use primitives::LocalAccess;
pub use primitives::LocalAssigment;
pub use statements::BodyStatement;
pub use statements::CoreStament;

pub trait PrimitiveOperation<T>:
    BitAnd<T, Output = T>
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
    + Decrement<T, Output = T>
{
}

macro_rules! primitive_operation_impl {
    ($($t:ty)*) => ($(
        impl PrimitiveOperation<$t> for $t {}
    )*)
}

primitive_operation_impl! {u8 u16 u32 u64 i8 i16 i32 i64}

pub trait AbstactSyntaxNode {
    type Output;
    type Context;

    fn eval(&self, context: &mut Self::Context) -> Self::Output;
}

static NODE_ID: AtomicU64 = AtomicU64::new(1);

pub fn alloc_id() -> u64 {
    NODE_ID.fetch_add(1, Ordering::Relaxed)
}

pub type Node<C, T> = Box<dyn AbstactSyntaxNode<Context = C, Output = T>>;
