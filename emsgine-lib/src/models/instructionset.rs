pub trait InstructionNamespace {}

pub trait MnemonicInstruction: InstructionNamespace {
    fn mnemonic<'a>(&self) -> &'a str;
}
