use crate::lookup::Lookup;
use super::bytes::DataWordSized;

pub trait InstructionNamespace {}

pub trait MnemonicInstruction: InstructionNamespace {
    fn mnemonic<'a>(&self) -> &'a str;
}

pub trait FormatInstruction: InstructionNamespace {
    fn formatstr<'a>(&self) -> &'a str;
    fn format<'a>(&self, ltbl: &'a dyn Lookup<&str, DataWordSized>) -> String;
}
