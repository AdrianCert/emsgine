use std::marker::PhantomData;

use super::{CpuContext, PointerContext};
use emsgine_lib::lookup::Lookup;
use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::models::instructionset::InstructionNamespace;

pub enum DecoderResult<'a, N, P>
where
    N: InstructionNamespace,
    P: Lookup<&'a str, DataWordSized>,
{
    DecodedRecord {
        addr: usize,
        bytes: Vec<u8>,
        size: usize,
        data: (N, P),
    },

    Failure {
        addr: usize,
        bytes: Vec<u8>,
    },

    Critical {
        addr: usize,
    },

    Pth(&'a PhantomData<P>),
}

pub trait InstructionDecoder<'a> {
    type Context: CpuContext;
    type Instruction: InstructionNamespace;
    type InstructionParameters: Lookup<&'a str, DataWordSized>;

    fn decode(
        &self,
        context: &mut Self::Context,
    ) -> DecoderResult<'a, Self::Instruction, Self::InstructionParameters> {
        let pc = self.current_addr(context);
        let pc_raw = &pc.as_raw_value();
        let fw = context.memory_pull(pc, self.word_size());
        let dr = self.analizer(&fw);
        if dr.is_ok() {
            return DecoderResult::DecodedRecord {
                addr: *pc_raw,
                size: self.word_size(),
                bytes: fw,
                data: dr.ok().unwrap(),
            };
        }

        let de = dr.err().unwrap();
        if 0 == de {
            return DecoderResult::Failure {
                addr: *pc_raw,
                bytes: fw,
            };
        }

        let fw = context.memory_pull(self.current_addr(context), de.into());
        let dr = self.analizer(&fw);
        if dr.is_ok() {
            return DecoderResult::DecodedRecord {
                addr: *pc_raw,
                size: de as usize,
                bytes: fw,
                data: dr.ok().unwrap(),
            };
        }

        DecoderResult::Critical { addr: *pc_raw }
    }

    fn analizer(&self, data: &[u8])
        -> Result<(Self::Instruction, Self::InstructionParameters), u8>;
    fn current_addr(
        &self,
        context: &Self::Context,
    ) -> <<Self as InstructionDecoder<'a>>::Context as CpuContext>::Pointer;
    fn word_size(&self) -> usize;
}
