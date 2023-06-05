use crate::contexts::ContextParameter;

pub enum ContextValue<'a, T> {
    Immediate(T),
    Parameter(&'a str),
}

impl<'a, T> ContextValue<'a, T> {
    pub fn resolve<P>(&self, context: &P) -> Option<T>
    where
        P: ContextParameter<Output = T>,
        T: Copy,
    {
        match self {
            ContextValue::Immediate(value) => Some(*value),
            ContextValue::Parameter(value) => context.param(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ContextParameter;
    use super::ContextValue;

    #[derive(Debug)]
    pub struct BasicsParameter<'a, T> {
        pub parameters: Vec<(&'a str, T)>,
    }

    impl<'a, T> ContextParameter for BasicsParameter<'a, T>
    where
        T: Copy,
    {
        type Output = T;

        fn param(&self, value: &str) -> Option<Self::Output> {
            // let items = ;
            for (key, param) in &self.parameters {
                if value.cmp(key) == core::cmp::Ordering::Equal {
                    return Some(*param);
                }
            }
            None
        }
    }

    #[test]
    fn usablity_param() {
        let param_conext_1 = BasicsParameter::<usize> {
            parameters: vec![("test", 12_usize), ("value", 2_usize)],
        };
        let param_conext_2 = BasicsParameter::<usize> {
            parameters: vec![("test", 11_usize), ("value", 3_usize)],
        };
        let value_a = ContextValue::<usize>::Immediate(12);
        let value_b = ContextValue::<usize>::Parameter("value");

        assert_eq!(value_a.resolve(&param_conext_1), Some(12_usize));
        assert_eq!(value_a.resolve(&param_conext_1), Some(12_usize));

        assert_eq!(value_b.resolve(&param_conext_1), Some(2_usize));
        assert_eq!(value_b.resolve(&param_conext_2), Some(3_usize));

        assert_eq!(value_b.resolve(&param_conext_2), Some(3_usize));
    }
}
