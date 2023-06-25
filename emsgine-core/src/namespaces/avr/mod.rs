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
    type InstructionParameters = Vec<(&'a str, emsgine_lib::models::bytes::DataWordSized)>;

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
        data: &[u8],
    ) -> Result<(Self::Instruction, Self::InstructionParameters), u8> {
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
    use crate::contexts::decoder::DecoderResult;
    use crate::contexts::loader;
    use crate::contexts::CpuContext;
    use crate::contexts::InstructionDecoder;
    use crate::memory::MemoryDevice;
    use std::fmt::Write;

    macro_rules! prog_write {
        ($cpu:expr, $adr:expr, $($val:expr)+) => {
            $cpu.memory_push(AddressPointer::ProgramSpace($adr), vec![$($val,)+])
        };
    }

    #[test]
    fn test_dism() {
        dissable_banch(0x039c / 2, |cpu| {
            let path = r"C:\Users\adria\Documents\Arduino\Blink\Blink.ino.hex";
            loader::load_memory_from_hexfile(path, cpu.mprg.clone());
        });
    }

    #[test]
    fn test_dism_flow_hexfile() {
        dissable_banch(0x12, |cpu| {
            prog_write!(cpu, 0x00, 0x80 0x91 0x00 0x00);
            prog_write!(cpu, 0x04, 0x61 0xe0);
            prog_write!(cpu, 0x06, 0x0e 0x94 0x00 0x00);
            prog_write!(cpu, 0x0a, 0x68 0xee);
            prog_write!(cpu, 0x0c, 0x73 0xe0);
            prog_write!(cpu, 0x0e, 0x80 0xe0);
            prog_write!(cpu, 0x10, 0x90 0xe0);
            prog_write!(cpu, 0x12, 0x0e 0x94 0x00 0x00);
            prog_write!(cpu, 0x16, 0x80 0x91 0x00 0x00);
            prog_write!(cpu, 0x1a, 0x60 0xe0);
            prog_write!(cpu, 0x1c, 0x0e 0x94 0x00 0x00);
            prog_write!(cpu, 0x20, 0x68 0xee);
            prog_write!(cpu, 0x22, 0x73 0xe0);
            prog_write!(cpu, 0x24, 0x80 0xe0);
            prog_write!(cpu, 0x26, 0x90 0xe0);
            prog_write!(cpu, 0x28, 0x0e 0x94 0x00 0x00);
            prog_write!(cpu, 0x2c, 0x08 0x95);
        });
    }

    fn dissable_banch<F>(loops: i32, fn_load: F)
    where
        F: FnOnce(&mut CentralProcessUnit),
    {
        let decoder = DecoderDevice {};
        let mut cpu = CentralProcessUnit {
            mram: Rc::new(MemoryDevice::from(vec![0; 0x20])),
            mmap: MemoryMap {
                reg_page: 0,
                io_space: 32,
                data_space: 60,
                overflow: 0xff3,
            },
            stbl: SymbolTable::create(vec![("PC", (AddressPointer::MemorySpace(0x0usize), 2))]),
            mspc: Rc::new(MemoryDevice::from(vec![0; 0xf])),
            mprg: Rc::new(MemoryDevice::from(vec![0; 0xfff])),
        };
        fn_load(&mut cpu);

        // println!("cpu sate: {cpu:?}", cpu = cpu);

        let mut x = 0;
        while x <= loops {
            // while x <= 0x30 / 2 {
            let decoded = decoder.decode(&mut cpu);

            // println! {"{decoded:?}"}

            if let DecoderResult::DecodedRecord {
                addr,
                bytes,
                data: (instruction, operands),
                size,
            } = decoded
            {
                println!(
                    "{addr:4x}:\t{:15} {}",
                    bytes.iter().fold(String::new(), |mut res, num| {
                        write!(&mut res, "{num:02x} ").unwrap();
                        res
                    }),
                    instruction.format(&operands)
                );
                cpu.states_set("PC", DataWordSized::DataSizeWord((addr + size) as u16));
            } else {
                cpu.states_update("PC", |x| DataWordSized::DataSizeWord(x.as_u16() + 2));
            }
            x += 1;
        }
    }
}
