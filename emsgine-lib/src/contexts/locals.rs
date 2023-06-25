use lazy_static::lazy_static;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

use crate::{lookup::Lookup, models::bytes::DataWordSized};

// ██╗      ██████╗  ██████╗ █████╗ ██╗     ███████╗    ███████╗███╗   ██╗██╗   ██╗
// ██║     ██╔═══██╗██╔════╝██╔══██╗██║     ██╔════╝    ██╔════╝████╗  ██║██║   ██║
// ██║     ██║   ██║██║     ███████║██║     ███████╗    █████╗  ██╔██╗ ██║██║   ██║
// ██║     ██║   ██║██║     ██╔══██║██║     ╚════██║    ██╔══╝  ██║╚██╗██║╚██╗ ██╔╝
// ███████╗╚██████╔╝╚██████╗██║  ██║███████╗███████║    ███████╗██║ ╚████║ ╚████╔╝
// ╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚══════╝╚═╝  ╚═══╝  ╚═══╝
// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=locals%20env

pub struct LocalEnvironment<V> {
    locals: RefCell<HashMap<String, V>>,
}

impl<V> LocalEnvironment<V>
where
    V: Copy,
{
    pub fn new() -> Self {
        LocalEnvironment {
            locals: RefCell::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &str, value: V) {
        self.locals.borrow_mut().insert(key.into(), value);
    }

    pub fn get(&self, key: &String) -> Option<V> {
        if let Some(value) = self.locals.borrow().get(key) {
            return Some(*value);
        }
        None
    }
}

impl<V> Default for LocalEnvironment<V>
where
    V: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Value, L> From<&L> for LocalEnvironment<Value>
where
    Value: Copy,
    L: Lookup<&'a str, Value>,
{
    fn from(value: &L) -> Self {
        let local = LocalEnvironment::new();
        for field in value.fields() {
            local.set(field, *value.lookup(field).unwrap())
        }

        local
    }
}

// ██╗      ██████╗  ██████╗ █████╗ ██╗     ███████╗    ███╗   ███╗ ██████╗ ██████╗
// ██║     ██╔═══██╗██╔════╝██╔══██╗██║     ██╔════╝    ████╗ ████║██╔════╝ ██╔══██╗
// ██║     ██║   ██║██║     ███████║██║     ███████╗    ██╔████╔██║██║  ███╗██████╔╝
// ██║     ██║   ██║██║     ██╔══██║██║     ╚════██║    ██║╚██╔╝██║██║   ██║██╔══██╗
// ███████╗╚██████╔╝╚██████╗██║  ██║███████╗███████║    ██║ ╚═╝ ██║╚██████╔╝██║  ██║
// ╚══════╝ ╚═════╝  ╚═════╝╚═╝  ╚═╝╚══════╝╚══════╝    ╚═╝     ╚═╝ ╚═════╝ ╚═╝  ╚═╝
// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=locals%20mgr

pub struct LocalEnvironmentManager<V> {
    stack: RefCell<Vec<Arc<LocalEnvironment<V>>>>,
}

impl LocalEnvironmentManager<DataWordSized> {
    pub fn new() -> Self {
        LocalEnvironmentManager {
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn new_local(&self) -> Arc<LocalEnvironment<DataWordSized>> {
        let local = Arc::new(LocalEnvironment::new());
        self.stack.borrow_mut().push(local.clone());
        local
    }

    pub fn push_local(&self, local: Arc<LocalEnvironment<DataWordSized>>) {
        self.stack.borrow_mut().push(local);
    }

    pub fn drop_local(&self) -> Arc<LocalEnvironment<DataWordSized>> {
        self.stack.borrow_mut().pop().unwrap()
    }

    pub fn head(&self) -> usize {
        self.stack.borrow().len() - 1
    }

    pub fn variable_set(&self, key: &str, value: DataWordSized) {
        let head = self.head();
        let local_scope = &self.stack.borrow()[head];
        local_scope.set(key, value)
    }

    pub fn variable_get(&self, key: &String) -> Option<DataWordSized> {
        for cscope in (0..=self.head()).rev() {
            let local_scope = &self.stack.borrow()[cscope];
            if let Some(variable) = local_scope.get(key) {
                return Some(variable);
            }
        }
        None
    }
}

impl Default for LocalEnvironmentManager<DataWordSized> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for LocalEnvironmentManager<DataWordSized> {}

// ███████╗ ██████╗ ██████╗ ██████╗ ███████╗███████╗
// ██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔════╝
// ███████╗██║     ██║   ██║██████╔╝█████╗  ███████╗
// ╚════██║██║     ██║   ██║██╔═══╝ ██╔══╝  ╚════██║
// ███████║╚██████╗╚██████╔╝██║     ███████╗███████║
// ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚══════╝╚══════╝
// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=scopes

lazy_static! {
    pub static ref SCOPES: LocalEnvironmentManager<DataWordSized> = {
        LocalEnvironmentManager {
            stack: RefCell::new(vec![Arc::new(LocalEnvironment::new())]),
        }
    };
}
