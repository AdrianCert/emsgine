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

pub struct LocalEnviroment<V> {
    locals: RefCell<HashMap<String, V>>,
}

impl<V> LocalEnviroment<V>
where
    V: Copy,
{
    pub fn new() -> Self {
        LocalEnviroment {
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

impl<V> Default for LocalEnviroment<V>
where
    V: Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Value, L> From<&L> for LocalEnviroment<Value>
where
    Value: Copy,
    L: Lookup<&'a str, Value>,
{
    fn from(value: &L) -> Self {
        let local = LocalEnviroment::new();
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

pub struct LocalEnviromentManager<V> {
    stack: RefCell<Vec<Arc<LocalEnviroment<V>>>>,
}

impl LocalEnviromentManager<DataWordSized> {
    pub fn new() -> Self {
        LocalEnviromentManager {
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn new_local(&self) -> Arc<LocalEnviroment<DataWordSized>> {
        let local = Arc::new(LocalEnviroment::new());
        self.stack.borrow_mut().push(local.clone());
        local
    }

    pub fn push_local(&self, local: Arc<LocalEnviroment<DataWordSized>>) {
        self.stack.borrow_mut().push(local);
    }

    pub fn drop_local(&self) -> Arc<LocalEnviroment<DataWordSized>> {
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

impl Default for LocalEnviromentManager<DataWordSized> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl Sync for LocalEnviromentManager<DataWordSized> {}

// ███████╗ ██████╗ ██████╗ ██████╗ ███████╗███████╗
// ██╔════╝██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔════╝
// ███████╗██║     ██║   ██║██████╔╝█████╗  ███████╗
// ╚════██║██║     ██║   ██║██╔═══╝ ██╔══╝  ╚════██║
// ███████║╚██████╗╚██████╔╝██║     ███████╗███████║
// ╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚══════╝╚══════╝
// https://patorjk.com/software/taag/#p=display&h=0&v=1&f=ANSI%20Shadow&t=scopes

lazy_static! {
    pub static ref SCOPES: LocalEnviromentManager<DataWordSized> = {
        LocalEnviromentManager {
            stack: RefCell::new(vec![Arc::new(LocalEnviroment::new())]),
        }
    };
}
