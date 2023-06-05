use super::CpuContext;
use emsgine_lib::lookup::Lookup;
use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::models::instructionset::InstructionNamespace;

pub trait InstructionDecoder<'a> {
    type Context: CpuContext;
    type Instruction: InstructionNamespace;
    type InstructionParameters<'ins>: Lookup<&'ins str, DataWordSized>;

    fn decode(
        &self,
        context: &mut Self::Context,
    ) -> Option<(Self::Instruction, Self::InstructionParameters<'a>)> {
        let fw = context.memory_pull(self.current_addr(context), self.word_size());
        let dr = self.analizer(fw);
        if dr.is_ok() {
            return dr.ok();
        }

        let de = dr.err().unwrap();
        if 0 == de {
            return None;
        }

        self.analizer(context.memory_pull(self.current_addr(context), de.into()))
            .ok()
    }

    fn analizer(
        &self,
        data: Vec<u8>,
    ) -> Result<(Self::Instruction, Self::InstructionParameters<'a>), u8>;
    fn current_addr(
        &self,
        context: &Self::Context,
    ) -> <<Self as InstructionDecoder<'a>>::Context as CpuContext>::Pointer;
    fn word_size(&self) -> usize;
}
