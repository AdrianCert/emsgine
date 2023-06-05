pub trait ContextParameter {
    type Output;

    fn param(&self, value: &str) -> Option<Self::Output>;
}
