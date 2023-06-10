pub mod ast;
mod decoder;
pub mod instructions;
pub use instructions::AvrInstructionSet;
mod cpu;

use crate::contexts::CpuContext;
use crate::contexts::InstructionDecoder;
pub use cpu::ENDIANNESS;
pub use cpu::{AddressPointer, CentralProcessUnit};
use decoder::decode;

struct DecoderDevice {}

impl<'a> InstructionDecoder<'a> for DecoderDevice {
    type Context = CentralProcessUnit;
    type Instruction = AvrInstructionSet;
    type InstructionParameters<'ins> = Vec<(&'ins str, emsgine_lib::models::bytes::DataWordSized)>;

    fn word_size(&self) -> usize {
        2
    }

    fn current_addr(
        &self,
        context: &Self::Context,
    ) -> <<Self as InstructionDecoder<'a>>::Context as crate::contexts::CpuContext>::Pointer {
        AddressPointer::ProgramSpace(context.states_get("PC").as_u16().into())
    }

    fn analizer(
        &self,
        data: Vec<u8>,
    ) -> Result<(Self::Instruction, Self::InstructionParameters<'a>), u8> {
        let bytes = data
            .chunks(2)
            .map(|chk| ENDIANNESS.compose(chk.to_vec()).as_u16())
            .collect::<Vec<u16>>();
        decode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use emsgine_lib::models::bytes::DataWordSized;
    use emsgine_lib::models::instructionset::FormatInstruction;

    use super::cpu::MemoryMap;
    use super::cpu::SymbolTable;
    use super::AddressPointer;
    use super::CentralProcessUnit;
    use super::DecoderDevice;
    use crate::contexts::loader;
    use crate::contexts::CpuContext;
    use crate::contexts::InstructionDecoder;
    use crate::memory::MemoryDevice;

    #[test]
    fn decoder() {
        let path = r"C:\Users\adria\Documents\Arduino\Blink\Blink.ino.hex";

        let decoder = DecoderDevice {};
        let mut cpu = CentralProcessUnit {
            mram: Rc::new(MemoryDevice::from(vec![0; 0xff3])),
            mmap: MemoryMap {
                reg_page: 0,
                io_space: 32,
                data_space: 60,
                overflow: 0xff3,
            },
            stbl: SymbolTable::create(vec![("PC", (AddressPointer::MemorySpace(0x1usize), 2))]),
            mspc: Rc::new(MemoryDevice::from(vec![0; 0xf])),
            mprg: Rc::new(MemoryDevice::from(vec![0; 0xfff])),
        };

        loader::load_memory_from_hexfile(path, cpu.mprg.clone());
        println!("cpu sate: {cpu:?}", cpu = cpu);

        let mut x = 0;
        while x <= 0x039c / 2 {
            let decoded = decoder.decode(&mut cpu);
            let addr = cpu.states_get("PC");
            cpu.states_update("PC", |x| DataWordSized::DataSizeWord(x.as_u16() + 2));
            let mem = cpu.memory_pull(AddressPointer::ProgramSpace(addr.as_u16().into()), 2);
            let val1 = mem[0];
            let val2 = mem[1];

            if let Some(ins) = decoded {
                let (ins, ops) = ins;
                println!(
                    "0x{:04x} {:02x} {:02x} >> {}",
                    addr.as_u16(),
                    val1,
                    val2,
                    ins.format(&ops)
                );
            }
            x += 1;
        }
    }
}
