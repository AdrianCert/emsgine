use super::{AvrInstructionSet, CentralProcessUnit};
use crate::{ast::cpu::registers::RegisterIndirectAccess, contexts::CpuContext};
use chain_cmp::chmp;
use emsgine_lib::{
    ast::{AbstactSyntaxNode, CoreStament, LocalAccess, Node},
    bitwise::BitSel,
    flag_eval,
    models::{
        bytes::DataWordSized,
        instructionset::{AstInstruction, MnemonicInstruction},
        portable::BoxPorting,
    },
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CPU_LOCALS: Vec<(&'static str, DataWordSized)> = {
        vec![
            ("FLAG_C_POSITION", DataWordSized::DataSizeByte(0)),
            ("FLAG_Z_POSITION", DataWordSized::DataSizeByte(1)),
            ("FLAG_N_POSITION", DataWordSized::DataSizeByte(2)),
            ("FLAG_V_POSITION", DataWordSized::DataSizeByte(3)),
            ("FLAG_S_POSITION", DataWordSized::DataSizeByte(4)),
            ("FLAG_H_POSITION", DataWordSized::DataSizeByte(5)),
            ("FLAG_T_POSITION", DataWordSized::DataSizeByte(6)),
            ("FLAG_I_POSITION", DataWordSized::DataSizeByte(7)),
        ]
    };
}

macro_rules! instruction_ast {
    ($inst:expr , $func:expr) => {
        CoreStament::new($func, Some(format!("RAW_AST_{}", $inst.mnemonic()))).porting_box()
    };
}

pub fn opr_adc(context: &mut CentralProcessUnit) {
    let c_flag = context.states_get("SREG").as_u8().bitsel(1);
    let d_reg = LocalAccess::<DataWordSized, CentralProcessUnit>::new("rds")
        .eval(context)
        .as_u8() as usize;
    let r_reg = RegisterIndirectAccess::new(
        LocalAccess::<DataWordSized, CentralProcessUnit>::new("rsr").porting_box(),
    );
    // casting up to prevent overflow
    let r_val = r_reg.eval(context) as u16;
    let d_val = context.register_get(d_reg) as u16;
    let result = r_val + d_val + c_flag as u16;
    let r_final = result & 255;
    let mut new_sreg = context.states_get("SREG").as_u8() & 0xc0;
    new_sreg |= flag_eval! { 0x01 , result >= 256 }; // C_FLAG
    new_sreg |= flag_eval! { 0x02 , r_final == 0 }; // Z_FLAG
    new_sreg |= flag_eval! { 0x04 , r_final & 128 > 0 }; // N_FLAG
    new_sreg |= flag_eval! { 0x08 , (r_final ^ d_val) & (r_final ^ r_val) & 128 > 0 };
    // V_FLAG
    new_sreg |= flag_eval! { 0x10 , chmp![0 < new_sreg & 0xc < 0xc ]}; // S_FLAG
    new_sreg |= flag_eval! { 0x20 , result & 0x1f > 0xf}; // H_FLAG

    context.states_set("SREG", new_sreg.into());
    context.register_set(d_reg, r_final as u8)
}

impl AstInstruction for AvrInstructionSet {
    type Context = CentralProcessUnit;

    type Output = ();
    fn get_ast(&self) -> Node<Self::Context, Self::Output> {
        match self {
            AvrInstructionSet::AddWithCarry => instruction_ast! { self, opr_adc },
            AvrInstructionSet::Compare => instruction_ast! { self, opr_adc },
            AvrInstructionSet::CompareSkipIfEqual => instruction_ast! { self, opr_adc },
            AvrInstructionSet::SubtractWithoutCarry => todo!(),
            AvrInstructionSet::Add => todo!(),
            AvrInstructionSet::CompareWithCarry => todo!(),
            AvrInstructionSet::FractionalMultiplyUnsigned => todo!(),
            AvrInstructionSet::MultiplySignedWithUnsigned => todo!(),
            AvrInstructionSet::FractionalMultiplySigned => todo!(),
            AvrInstructionSet::FractionalMultiplySignedWithUnsigned => todo!(),
            AvrInstructionSet::CopyRegisterWord => todo!(),
            AvrInstructionSet::MultiplySigned => todo!(),
            AvrInstructionSet::NoOperation => todo!(),
            AvrInstructionSet::SubtractWithCarry => todo!(),
            AvrInstructionSet::AddWithImmediate => todo!(),
            AvrInstructionSet::SubtractImmediateWord => todo!(),
            AvrInstructionSet::ArithmeticShiftRight => todo!(),
            AvrInstructionSet::BitClearInSREG => todo!(),
            AvrInstructionSet::Break => todo!(),
            AvrInstructionSet::ExtendedLoadProgramMemory => todo!(),
            AvrInstructionSet::LoadProgramMemory => todo!(),
            AvrInstructionSet::Sleep => todo!(),
            AvrInstructionSet::StoreMemoryProgram => todo!(),
            AvrInstructionSet::StoreMemoryProgramPostIncrementZ => todo!(),
            AvrInstructionSet::WatchdogReset => todo!(),
            AvrInstructionSet::BitSet => todo!(),
            AvrInstructionSet::ExtendedIndirectJump => todo!(),
            AvrInstructionSet::IndirectJump => todo!(),
            AvrInstructionSet::ExtendedIndirectCallSubroutine => todo!(),
            AvrInstructionSet::ReturnFromInterrupt => todo!(),
            AvrInstructionSet::IndirectCallSubroutine => todo!(),
            AvrInstructionSet::ReturnFromSubroutine => todo!(),
            AvrInstructionSet::Call => todo!(),
            AvrInstructionSet::Complement => todo!(),
            AvrInstructionSet::ComplementTwo => todo!(),
            AvrInstructionSet::Decrement => todo!(),
            AvrInstructionSet::DataEncryptionStandart => todo!(),
            AvrInstructionSet::Increment => todo!(),
            AvrInstructionSet::SwapNibbles => todo!(),
            AvrInstructionSet::Jump => todo!(),
            AvrInstructionSet::LogicalShiftRight => todo!(),
            AvrInstructionSet::RotateRightThroughCarry => todo!(),
            AvrInstructionSet::ClearIOBit => todo!(),
            AvrInstructionSet::SetBitIoSpace => todo!(),
            AvrInstructionSet::SkipIfIoBitCleared => todo!(),
            AvrInstructionSet::SkipIfIoBitSet => todo!(),
            AvrInstructionSet::ExtendedLoadProgramMemoryZ => todo!(),
            AvrInstructionSet::ExtendedLoadProgramMemoryZplus => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ => todo!(),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ => todo!(),
            AvrInstructionSet::LoadDirectFromDataSpace => todo!(),
            AvrInstructionSet::LoadProgramMemoryZ => todo!(),
            AvrInstructionSet::LoadProgramMemoryZplus => todo!(),
            AvrInstructionSet::PopRegisterFromStack => todo!(),
            AvrInstructionSet::LoadAndClear => todo!(),
            AvrInstructionSet::LoadAndSet => todo!(),
            AvrInstructionSet::LoadAndToggle => todo!(),
            AvrInstructionSet::PushRegisterToStack => todo!(),
            AvrInstructionSet::StoreIndirectDataSpaceIndexX => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ => todo!(),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ => todo!(),
            AvrInstructionSet::StoreDirectDataSpace => todo!(),
            AvrInstructionSet::Exchange => todo!(),
            AvrInstructionSet::MultiplyUnsigned => todo!(),
            AvrInstructionSet::LogicalAnd => todo!(),
            AvrInstructionSet::LogicalExclusiveOr => todo!(),
            AvrInstructionSet::CopyRegister => todo!(),
            AvrInstructionSet::LogicalOr => todo!(),
            AvrInstructionSet::LogicalAndWithImmediate => todo!(),
            AvrInstructionSet::BitLoadInRegisterfromT => todo!(),
            AvrInstructionSet::BitStorefromRegister => todo!(),
            AvrInstructionSet::BranchSregBitCleared => todo!(),
            AvrInstructionSet::BranchSregBitSet => todo!(),
            AvrInstructionSet::SkipIfBitCleared => todo!(),
            AvrInstructionSet::SkipIfBitSet => todo!(),
            AvrInstructionSet::CompareWithImmediate => todo!(),
            AvrInstructionSet::LoadFromIOSpace => todo!(),
            AvrInstructionSet::StoreToIoSpace => todo!(),
            AvrInstructionSet::LoadImmediate => todo!(),
            AvrInstructionSet::LogicalOrWithImmediate => todo!(),
            AvrInstructionSet::RelativeCallSubroutine => todo!(),
            AvrInstructionSet::RelativeJump => todo!(),
            AvrInstructionSet::SubtractImmediateWithCarrySbi => todo!(),
            AvrInstructionSet::SubtractImmediate => todo!(),
            AvrInstructionSet::LoadIndirectWithDisplacementY => todo!(),
            AvrInstructionSet::StoreIndirectWithDisplacementY => todo!(),
            AvrInstructionSet::LoadIndirectWithDisplacementZ => todo!(),
            AvrInstructionSet::StoreIndirectWithDisplacementZ => todo!(),
            AvrInstructionSet::LoadStoreIndirectWithHightDisplacement => todo!(),
            AvrInstructionSet::Invalid => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use emsgine_lib::{
        ast::AbstactSyntaxNode,
        contexts::locals::{LocalEnviroment, SCOPES},
        models::bytes::DataWordSized,
    };

    use crate::{
        ast::cpu::instruction::CpuInstruction,
        contexts::{CpuContext, MemoryContext},
        memory::MemoryDevice,
        namespaces::avr::{
            ast::CPU_LOCALS,
            cpu::{MemoryMap, SymbolTable},
            AddressPointer, AvrInstructionSet, CentralProcessUnit,
        },
    };

    fn setup_cpu() -> CentralProcessUnit {
        CentralProcessUnit {
            mram: Rc::new(MemoryDevice::from(vec![0; 0xff3])),
            mmap: MemoryMap {
                reg_page: 0,
                io_space: 32,
                data_space: 60,
                overflow: 0xff3,
            },
            stbl: SymbolTable::create(vec![
                ("PC", (AddressPointer::SpecialSpace(0x0_usize), 2)),
                ("SREG", (AddressPointer::MemorySpace(0x5f_usize), 1)),
            ]),
            mspc: Rc::new(MemoryDevice::from(vec![0; 0xf])),
            mprg: Rc::new(MemoryDevice::from(vec![0; 0xfff])),
        }
    }

    #[test]
    fn test_cpu_instruction_one() {
        let mut cpu = setup_cpu();

        let (inst_id, inst_arg) = (
            AvrInstructionSet::AddWithCarry,
            vec![
                ("rsr", DataWordSized::DataSizeByte(1)),
                ("rds", DataWordSized::DataSizeByte(0)),
            ],
        );

        cpu.register_set(0, 25);
        cpu.register_set(1, 1);

        let ins_ast: CpuInstruction<CentralProcessUnit, ()> =
            CpuInstruction::from_instruction(inst_id, &inst_arg);

        let cpu_local: Arc<LocalEnviroment<DataWordSized>> = Arc::new({
            let env = LocalEnviroment::<DataWordSized>::new();
            for (var, val) in CPU_LOCALS.iter() {
                env.set(var, *val);
            }
            env
        });

        SCOPES.push_local(cpu_local);

        println!("reg page: {:?}", cpu.mram.read_bytes(0_usize, 32));
        println!("SREG: {:?}", cpu.states_get("SREG"));
        ins_ast.eval(&mut cpu);
        println!("reg page: {:?}", cpu.mram.read_bytes(0_usize, 32));
        println!("SREG: {:?}", cpu.states_get("SREG"));
    }
}
