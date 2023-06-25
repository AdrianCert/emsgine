use super::{AvrInstructionSet, CentralProcessUnit};
use crate::{
    ast::{
        cpu::registers::{RegisterIndirectAccess, RegisterIndirectAssign},
        provider::AstFactoryProvider,
    },
    contexts::CpuContext,
};
use chain_cmp::chmp;
use emsgine_lib::{
    ast::{AbstactSyntaxNode, BodyStatement, CoreStament, LocalAccess, Node},
    bitwise::BitSel,
    contexts::locals::SCOPES,
    flag_eval,
    models::{bytes::DataWordSized, instructionset::MnemonicInstruction, portable::BoxPorting},
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

macro_rules! get_local {
    ($context:expr , $val:expr) => {
        LocalAccess::<CentralProcessUnit, DataWordSized>::new($val).eval($context)
    };
}

macro_rules! reg_assing_from_local {
    ($reg:expr, $val:expr) => {
        RegisterIndirectAssign::new(
            LocalAccess::<CentralProcessUnit, DataWordSized>::new($reg).porting_box(),
            LocalAccess::<CentralProcessUnit, DataWordSized>::new($val).porting_box(),
        )
        .porting_box()
    };
}

pub fn opr_adc(context: &mut CentralProcessUnit) {
    let c_flag = context.states_get("SREG").as_u8().bitsel(1);
    let d_reg = get_local![context, "rds"].as_u16() as usize;
    let r_reg = RegisterIndirectAccess::new(
        LocalAccess::<CentralProcessUnit, DataWordSized>::new("rsr").porting_box(),
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

    SCOPES.variable_set("R", DataWordSized::DataSizeByte(r_final as u8));
    SCOPES.variable_set("R_SREG", new_sreg.into());
    // context.states_set("SREG", new_sreg.into());
    // context.register_set(d_reg, r_final as u8)
}

pub fn opr_add(context: &mut CentralProcessUnit) {
    let d_reg = get_local![context, "rds"].as_u16() as usize;
    let r_reg = RegisterIndirectAccess::new(
        LocalAccess::<CentralProcessUnit, DataWordSized>::new("rsr").porting_box(),
    );
    // casting up to prevent overflow
    let r_val = r_reg.eval(context) as u16;
    let d_val = context.register_get(d_reg) as u16;
    let result = r_val + d_val;
    let r_final = result & 255;
    let mut new_sreg = context.states_get("SREG").as_u8() & 0xc0;
    new_sreg |= flag_eval! { 0x01 , result >= 256 }; // C_FLAG
    new_sreg |= flag_eval! { 0x02 , r_final == 0 }; // Z_FLAG
    new_sreg |= flag_eval! { 0x04 , r_final & 128 > 0 }; // N_FLAG
    new_sreg |= flag_eval! { 0x08 , (r_final ^ d_val) & (r_final ^ r_val) & 128 > 0 };
    // V_FLAG
    new_sreg |= flag_eval! { 0x10 , chmp![0 < new_sreg & 0xc < 0xc ]}; // S_FLAG
    new_sreg |= flag_eval! { 0x20 , result & 0x1f > 0xf}; // H_FLAG

    SCOPES.variable_set("R", DataWordSized::DataSizeByte(r_final as u8));
    SCOPES.variable_set("R_SREG", new_sreg.into());
}
pub fn opr_cp(_context: &mut CentralProcessUnit) {}

struct AvrAstServerProvider {}

impl AstFactoryProvider<AvrInstructionSet, CentralProcessUnit> for AvrAstServerProvider {
    type Output = ();

    fn get_ast(
        &self,
        instruction: &AvrInstructionSet,
    ) -> Option<Node<CentralProcessUnit, Self::Output>> {
        match instruction {
            AvrInstructionSet::AddWithCarry => Some(
                BodyStatement::new(vec![
                    instruction_ast! { instruction, opr_adc },
                    reg_assing_from_local! { "rds", "R"},
                ])
                .porting_box(),
            ),
            AvrInstructionSet::Compare => Some(instruction_ast! { instruction, opr_cp }),
            AvrInstructionSet::CompareSkipIfEqual => {
                Some(instruction_ast! { instruction, opr_adc })
            }
            AvrInstructionSet::SubtractWithoutCarry => None,
            AvrInstructionSet::Add => Some(
                BodyStatement::new(vec![
                    instruction_ast! { instruction, opr_add },
                    reg_assing_from_local! { "rds", "R"},
                ])
                .porting_box(),
            ),
            AvrInstructionSet::CompareWithCarry => None,
            AvrInstructionSet::FractionalMultiplyUnsigned => None,
            AvrInstructionSet::MultiplySignedWithUnsigned => None,
            AvrInstructionSet::FractionalMultiplySigned => None,
            AvrInstructionSet::FractionalMultiplySignedWithUnsigned => None,
            AvrInstructionSet::CopyRegisterWord => None,
            AvrInstructionSet::MultiplySigned => None,
            AvrInstructionSet::NoOperation => None,
            AvrInstructionSet::SubtractWithCarry => None,
            AvrInstructionSet::AddWithImmediate => None,
            AvrInstructionSet::SubtractImmediateWord => None,
            AvrInstructionSet::ArithmeticShiftRight => None,
            AvrInstructionSet::BitClearInSREG => None,
            AvrInstructionSet::Break => None,
            AvrInstructionSet::ExtendedLoadProgramMemory => None,
            AvrInstructionSet::LoadProgramMemory => None,
            AvrInstructionSet::Sleep => None,
            AvrInstructionSet::StoreMemoryProgram => None,
            AvrInstructionSet::StoreMemoryProgramPostIncrementZ => None,
            AvrInstructionSet::WatchdogReset => None,
            AvrInstructionSet::BitSet => None,
            AvrInstructionSet::ExtendedIndirectJump => None,
            AvrInstructionSet::IndirectJump => None,
            AvrInstructionSet::ExtendedIndirectCallSubroutine => None,
            AvrInstructionSet::ReturnFromInterrupt => None,
            AvrInstructionSet::IndirectCallSubroutine => None,
            AvrInstructionSet::ReturnFromSubroutine => None,
            AvrInstructionSet::Call => None,
            AvrInstructionSet::Complement => None,
            AvrInstructionSet::ComplementTwo => None,
            AvrInstructionSet::Decrement => None,
            AvrInstructionSet::DataEncryptionStandart => None,
            AvrInstructionSet::Increment => None,
            AvrInstructionSet::SwapNibbles => None,
            AvrInstructionSet::Jump => None,
            AvrInstructionSet::LogicalShiftRight => None,
            AvrInstructionSet::RotateRightThroughCarry => None,
            AvrInstructionSet::ClearIOBit => None,
            AvrInstructionSet::SetBitIoSpace => None,
            AvrInstructionSet::SkipIfIoBitCleared => None,
            AvrInstructionSet::SkipIfIoBitSet => None,
            AvrInstructionSet::ExtendedLoadProgramMemoryZ => None,
            AvrInstructionSet::ExtendedLoadProgramMemoryZplus => None,
            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ => None,
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ => None,
            AvrInstructionSet::LoadDirectFromDataSpace => None,
            AvrInstructionSet::LoadProgramMemoryZ => None,
            AvrInstructionSet::LoadProgramMemoryZplus => None,
            AvrInstructionSet::PopRegisterFromStack => None,
            AvrInstructionSet::LoadAndClear => None,
            AvrInstructionSet::LoadAndSet => None,
            AvrInstructionSet::LoadAndToggle => None,
            AvrInstructionSet::PushRegisterToStack => None,
            AvrInstructionSet::StoreIndirectDataSpaceIndexX => None,
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX => None,
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX => None,
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY => None,
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY => None,
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ => None,
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ => None,
            AvrInstructionSet::StoreDirectDataSpace => None,
            AvrInstructionSet::Exchange => None,
            AvrInstructionSet::MultiplyUnsigned => None,
            AvrInstructionSet::LogicalAnd => None,
            AvrInstructionSet::LogicalExclusiveOr => None,
            AvrInstructionSet::CopyRegister => None,
            AvrInstructionSet::LogicalOr => None,
            AvrInstructionSet::LogicalAndWithImmediate => None,
            AvrInstructionSet::BitLoadInRegisterfromT => None,
            AvrInstructionSet::BitStorefromRegister => None,
            AvrInstructionSet::BranchSregBitCleared => None,
            AvrInstructionSet::BranchSregBitSet => None,
            AvrInstructionSet::SkipIfBitCleared => None,
            AvrInstructionSet::SkipIfBitSet => None,
            AvrInstructionSet::CompareWithImmediate => None,
            AvrInstructionSet::LoadFromIOSpace => None,
            AvrInstructionSet::StoreToIoSpace => None,
            AvrInstructionSet::LoadImmediate => None,
            AvrInstructionSet::LogicalOrWithImmediate => None,
            AvrInstructionSet::RelativeCallSubroutine => None,
            AvrInstructionSet::RelativeJump => None,
            AvrInstructionSet::SubtractImmediateWithCarrySbi => None,
            AvrInstructionSet::SubtractImmediate => None,
            AvrInstructionSet::LoadIndirectWithDisplacementY => None,
            AvrInstructionSet::StoreIndirectWithDisplacementY => None,
            AvrInstructionSet::LoadIndirectWithDisplacementZ => None,
            AvrInstructionSet::StoreIndirectWithDisplacementZ => None,
            AvrInstructionSet::LoadStoreIndirectWithHightDisplacement => None,
            // AvrInstructionSet::Invalid => None,
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::LinkedList, rc::Rc, sync::Arc};

    use emsgine_lib::{
        ast::AbstactSyntaxNode,
        contexts::locals::{LocalEnvironment, SCOPES},
        models::bytes::DataWordSized,
    };

    use crate::{
        ast::{
            cpu::instruction::{CpuInstruction, CpuInstructionFactory},
            provider::AstFactoryProvider,
        },
        contexts::{CpuContext, MemoryContext},
        memory::MemoryDevice,
        namespaces::avr::{
            ast::{AvrAstServerProvider, CPU_LOCALS},
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

        let ast_factory = CpuInstructionFactory {
            providers: {
                let mut list: LinkedList<
                    Box<dyn AstFactoryProvider<AvrInstructionSet, CentralProcessUnit, Output = ()>>,
                > = LinkedList::new();
                list.push_back(Box::new(AvrAstServerProvider {}));

                list
            },
        };

        let ins_ast: Option<CpuInstruction<CentralProcessUnit, ()>> =
            ast_factory.parse(&inst_id, &inst_arg);
        // CpuInstruction::from_instruction(inst_id, &inst_arg);

        let cpu_local: Arc<LocalEnvironment<DataWordSized>> = Arc::new({
            let env = LocalEnvironment::<DataWordSized>::new();
            for (var, val) in CPU_LOCALS.iter() {
                env.set(var, *val);
            }
            env
        });

        SCOPES.push_local(cpu_local);

        println!("reg page: {:?}", cpu.mram.read_bytes(0_usize, 32));
        println!("SREG: {:?}", cpu.states_get("SREG"));
        if let Some(ast) = ins_ast {
            ast.eval(&mut cpu);
        }
        println!("reg page: {:?}", cpu.mram.read_bytes(0_usize, 32));
        println!("SREG: {:?}", cpu.states_get("SREG"));
    }
}
