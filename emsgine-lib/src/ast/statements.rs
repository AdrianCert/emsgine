use super::alloc_id;
use super::AbstactSyntaxNode;
use super::Node;
use crate::contexts::locals::SCOPES;
use crate::models::portable::BoxPorting;

pub struct BodyStatement<C, T> {
    statements: Vec<Node<C, T>>,
}

impl<C, T> BodyStatement<C, T> {
    pub fn new(statements: Vec<Node<C, T>>) -> Self {
        BodyStatement { statements }
    }
}

impl<C, T> AbstactSyntaxNode for BodyStatement<C, T>
where
    T: Default,
{
    type Output = T;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        SCOPES.new_local();
        let mut statemnts = self.statements.iter().peekable();
        let mut result = Self::Output::default();
        while let Some(stament) = statemnts.next() {
            if statemnts.peek().is_none() {
                result = stament.eval(context);
            } else {
                stament.eval(context);
            }
        }
        SCOPES.drop_local();
        result
    }
}

impl<C, T> BoxPorting for BodyStatement<C, T> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}

pub struct CoreStament<C, T> {
    pub evaluator: fn(&mut C) -> T,
    pub name: String,
    _nid: u64,
}

impl<C, T> CoreStament<C, T> {
    pub fn new(evalutator: fn(&mut C) -> T, name: Option<String>) -> Self {
        let _nid = alloc_id();
        if let Some(name) = name {
            return CoreStament {
                evaluator: evalutator,
                name,
                _nid,
            };
        }

        CoreStament {
            evaluator: evalutator,
            name: format!("Core function {}", _nid),
            _nid,
        }
    }
}

impl<C, T> AbstactSyntaxNode for CoreStament<C, T>
where
    T: Default,
{
    type Output = T;
    type Context = C;

    fn eval(&self, context: &mut Self::Context) -> Self::Output {
        (self.evaluator)(context)
    }
}

impl<C, T> BoxPorting for CoreStament<C, T> {
    fn porting_box(self) -> Box<Self> {
        Box::new(self)
    }
}
