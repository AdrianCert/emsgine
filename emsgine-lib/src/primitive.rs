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
        pub parameters: Box<Vec<(&'a str, T)>>,
    }

    impl<'a, T> ContextParameter for BasicsParameter<'a, T>
    where
        T: Copy,
    {
        type Output = T;

        fn param(&self, value: &str) -> Option<Self::Output> {
            let items = self.parameters.as_ref();
            for (key, param) in items {
                if value.cmp(key) == core::cmp::Ordering::Equal {
                    return Some(param.clone());
                }
            }
            None
        }
    }

    #[test]
    fn usablity_param() {
        let param_conext_1 = BasicsParameter::<usize> {
            parameters: Box::new(vec![("test", 12 as usize), ("value", 2 as usize)]),
        };
        let param_conext_2 = BasicsParameter::<usize> {
            parameters: Box::new(vec![("test", 11 as usize), ("value", 3 as usize)]),
        };
        let value_a = ContextValue::<usize>::Immediate(12);
        let value_b = ContextValue::<usize>::Parameter("value");

        assert_eq!(value_a.resolve(&param_conext_1), Some(12 as usize));
        assert_eq!(value_a.resolve(&param_conext_1), Some(12 as usize));

        assert_eq!(value_b.resolve(&param_conext_1), Some(2 as usize));
        assert_eq!(value_b.resolve(&param_conext_2), Some(3 as usize));

        assert_eq!(value_b.resolve(&param_conext_2), Some(3 as usize));
    }
}
