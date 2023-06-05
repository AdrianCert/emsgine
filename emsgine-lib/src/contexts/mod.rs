pub mod parameters;

pub use parameters::ContextParameter;

pub trait Context {
    // fn change_context<T>(&self, param: T) -> Self;
}
