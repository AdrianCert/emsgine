// generated-code: 2023-06-03T10:04:00.137529

use emsgine_lib::models::instructionset::{InstructionNamespace, MnemonicInstruction};

#[derive(Debug)]
pub enum AvrInstructionSet {
    AddWithCarry, //ADC Rd,Rr
    Compare, //CP Rd,Rr
    CompareSkipIfEqual, //CPSE Rd,Rr
    SubtractWithoutCarry, //SUB Rd,Rr
    Add, //ADD Rd,Rr
    CompareWithCarry, //CPC Rd,Rr
    FractionalMultiplyUnsigned, //FMUL Rd,Rr
    MultiplySignedWithUnsigned, //MULSU Rd,Rr
    FractionalMultiplySigned, //FMULS Rd,Rr
    FractionalMultiplySignedWithUnsigned, //FMULSU Rd,Rr
    CopyRegisterWord, //MOVW Rd+1:Rd,Rr+1:Rr
    MultiplySigned, //MULS Rd,Rr
    NoOperation, //NOP
    SubtractWithCarry, //SBC Rd,Rr
    AddWithImmediate, //ADIW Rd+1:Rd,K
    SubtractImmediateWord, //SBIW Rd+1:Rd,K
    ArithmeticShiftRight, //ASR Rd
    BitClearInSREG, //BCLR s
    Break, //BREAK
    ExtendedLoadProgramMemory, //ELPM
    LoadProgramMemory, //LPM
    Sleep, //SLEEP
    StoreMemoryProgram, //SPM
    StoreMemoryProgramPostIncrementZ, //SPM Z+
    WatchdogReset, //WDR
    BitSet, //BSET s
    ExtendedIndirectJump, //EIJMP
    IndirectJump, //IJMP
    ExtendedIndirectCallSubroutine, //EICALL
    ReturnFromInterrupt, //RETI
    IndirectCallSubroutine, //ICALL
    ReturnFromSubroutine, //RET
    Call, //CALL k
    Complement, //COM Rd
    ComplementTwo, //NEG Rd
    Decrement, //DEC Rd
    DataEncryptionStandart, //DES K
    Increment, //INC Rd
    SwapNibbles, //SWAP Rd
    Jump, //JMP k
    LogicalShiftRight, //LSR Rd
    RotateRightThroughCarry, //ROR Rd
    ClearIOBit, //CBI A,b
    SetBitIoSpace, //SBI A,b
    SkipIfIoBitCleared, //SBIC A,b
    SkipIfIoBitSet, //SBIS A,b
    ExtendedLoadProgramMemoryZ, //ELPM Rd, Z
    ExtendedLoadProgramMemoryZplus, //ELPM Rd, Z+
    LoadIndirectFromDataSpaceIndexX, //LD Rd, X
    LoadIndirectFromDataSpacePostIncrementX, //LD Rd, X+
    LoadIndirectFromDataSpacePreDecrementX, //LD Rd, -X
    LoadIndirectFromDataSpacePostIncrementY, //LD Rd, Y+
    LoadIndirectFromDataSpacePreDecrementY, //LD Rd, -Y
    LoadIndirectFromDataSpacePostIncrementZ, //LD Rd, Z+
    LoadIndirectFromDataSpacePreDecrementZ, //LD Rd, -Z
    LoadDirectFromDataSpace, //LDS Rd,k
    LoadProgramMemoryZ, //LPM Rd, Z
    LoadProgramMemoryZplus, //LPM Rd, Z+
    PopRegisterFromStack, //POP Rd
    LoadAndClear, //LAC Z,Rd
    LoadAndSet, //LAS Z,Rd
    LoadAndToggle, //LAT Z,Rd
    PushRegisterToStack, //PUSH Rr
    StoreIndirectDataSpaceIndexX, //ST X, Rr
    StoreIndirectDataSpacePostIncrementX, //ST X+, Rr
    StoreIndirectDataSpacePreDecrementX, //ST -X, Rr
    StoreIndirectDataSpacePostIncrementY, //ST Y+, Rr
    StoreIndirectDataSpacePreDecrementY, //ST -Y, Rr
    StoreIndirectDataSpacePostIncrementZ, //ST Z+, Rr
    StoreIndirectDataSpacePreDecrementZ, //ST -Z, Rr
    StoreDirectDataSpace, //STS k,Rr
    Exchange, //XCH Z,Rd
    MultiplyUnsigned, //MUL Rd,Rr
    LogicalAnd, //AND Rd,Rr
    LogicalExclusiveOr, //EOR Rd,Rr
    CopyRegister, //MOV Rd,Rr
    LogicalOr, //OR Rd,Rr
    LogicalAndWithImmediate, //ANDI Rd,K
    BitLoadInRegisterfromT, //BLD Rd,b
    BitStorefromRegister, //BST Rd,b
    BranchSregBitCleared, //BRBC s,k
    BranchSregBitSet, //BRBS s,k
    SkipIfBitCleared, //SBRC Rr,b
    SkipIfBitSet, //SBRS Rr,b
    CompareWithImmediate, //CPI Rd,K
    LoadFromIOSpace, //IN Rd,A
    StoreToIoSpace, //OUT A,Rr
    LoadImmediate, //LDI Rd,K
    LogicalOrWithImmediate, //ORI Rd,K
    RelativeCallSubroutine, //RCALL k
    RelativeJump, //RJMP k
    SubtractImmediateWithCarrySbi, //SBCI Rd,K
    SubtractImmediate, //SUBI Rd,K
    LoadIndirectWithDisplacementY, //LDD Rd, Y+q
    StoreIndirectWithDisplacementY, //STD Y+q, Rr
    LoadIndirectWithDisplacementZ, //LDD Rd, Z+q
    StoreIndirectWithDisplacementZ, //STD Z+q, Rr
    LoadStoreIndirectWithHightDisplacement, //STD Y+q, Rr
    Invalid
}

impl InstructionNamespace for AvrInstructionSet {}

impl MnemonicInstruction for AvrInstructionSet {
    fn mnemonic<'a>(&self) -> &'a str {
        match self {
            AvrInstructionSet::AddWithCarry => "ADC",
            AvrInstructionSet::Compare => "CP",
            AvrInstructionSet::CompareSkipIfEqual => "CPSE",
            AvrInstructionSet::SubtractWithoutCarry => "SUB",
            AvrInstructionSet::Add => "ADD",
            AvrInstructionSet::CompareWithCarry => "CPC",
            AvrInstructionSet::FractionalMultiplyUnsigned => "FMUL",
            AvrInstructionSet::MultiplySignedWithUnsigned => "MULSU",
            AvrInstructionSet::FractionalMultiplySigned => "FMULS",
            AvrInstructionSet::FractionalMultiplySignedWithUnsigned => "FMULSU",
            AvrInstructionSet::CopyRegisterWord => "MOVW",
            AvrInstructionSet::MultiplySigned => "MULS",
            AvrInstructionSet::NoOperation => "NOP",
            AvrInstructionSet::SubtractWithCarry => "SBC",
            AvrInstructionSet::AddWithImmediate => "ADIW",
            AvrInstructionSet::SubtractImmediateWord => "SBIW",
            AvrInstructionSet::ArithmeticShiftRight => "ASR",
            AvrInstructionSet::BitClearInSREG => "BCLR",
            AvrInstructionSet::Break => "BREAK",
            AvrInstructionSet::ExtendedLoadProgramMemory => "ELPM",
            AvrInstructionSet::LoadProgramMemory => "LPM",
            AvrInstructionSet::Sleep => "SLEEP",
            AvrInstructionSet::StoreMemoryProgram => "SPM",
            AvrInstructionSet::StoreMemoryProgramPostIncrementZ => "SPM",
            AvrInstructionSet::WatchdogReset => "WDR",
            AvrInstructionSet::BitSet => "BSET",
            AvrInstructionSet::ExtendedIndirectJump => "EIJMP",
            AvrInstructionSet::IndirectJump => "IJMP",
            AvrInstructionSet::ExtendedIndirectCallSubroutine => "EICALL",
            AvrInstructionSet::ReturnFromInterrupt => "RETI",
            AvrInstructionSet::IndirectCallSubroutine => "ICALL",
            AvrInstructionSet::ReturnFromSubroutine => "RET",
            AvrInstructionSet::Call => "CALL",
            AvrInstructionSet::Complement => "COM",
            AvrInstructionSet::ComplementTwo => "NEG",
            AvrInstructionSet::Decrement => "DEC",
            AvrInstructionSet::DataEncryptionStandart => "DES",
            AvrInstructionSet::Increment => "INC",
            AvrInstructionSet::SwapNibbles => "SWAP",
            AvrInstructionSet::Jump => "JMP",
            AvrInstructionSet::LogicalShiftRight => "LSR",
            AvrInstructionSet::RotateRightThroughCarry => "ROR",
            AvrInstructionSet::ClearIOBit => "CBI",
            AvrInstructionSet::SetBitIoSpace => "SBI",
            AvrInstructionSet::SkipIfIoBitCleared => "SBIC",
            AvrInstructionSet::SkipIfIoBitSet => "SBIS",
            AvrInstructionSet::ExtendedLoadProgramMemoryZ => "ELPM",
            AvrInstructionSet::ExtendedLoadProgramMemoryZplus => "ELPM",
            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ => "LD",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ => "LD",
            AvrInstructionSet::LoadDirectFromDataSpace => "LDS",
            AvrInstructionSet::LoadProgramMemoryZ => "LPM",
            AvrInstructionSet::LoadProgramMemoryZplus => "LPM",
            AvrInstructionSet::PopRegisterFromStack => "POP",
            AvrInstructionSet::LoadAndClear => "LAC",
            AvrInstructionSet::LoadAndSet => "LAS",
            AvrInstructionSet::LoadAndToggle => "LAT",
            AvrInstructionSet::PushRegisterToStack => "PUSH",
            AvrInstructionSet::StoreIndirectDataSpaceIndexX => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ => "ST",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ => "ST",
            AvrInstructionSet::StoreDirectDataSpace => "STS",
            AvrInstructionSet::Exchange => "XCH",
            AvrInstructionSet::MultiplyUnsigned => "MUL",
            AvrInstructionSet::LogicalAnd => "AND",
            AvrInstructionSet::LogicalExclusiveOr => "EOR",
            AvrInstructionSet::CopyRegister => "MOV",
            AvrInstructionSet::LogicalOr => "OR",
            AvrInstructionSet::LogicalAndWithImmediate => "ANDI",
            AvrInstructionSet::BitLoadInRegisterfromT => "BLD",
            AvrInstructionSet::BitStorefromRegister => "BST",
            AvrInstructionSet::BranchSregBitCleared => "BRBC",
            AvrInstructionSet::BranchSregBitSet => "BRBS",
            AvrInstructionSet::SkipIfBitCleared => "SBRC",
            AvrInstructionSet::SkipIfBitSet => "SBRS",
            AvrInstructionSet::CompareWithImmediate => "CPI",
            AvrInstructionSet::LoadFromIOSpace => "IN",
            AvrInstructionSet::StoreToIoSpace => "OUT",
            AvrInstructionSet::LoadImmediate => "LDI",
            AvrInstructionSet::LogicalOrWithImmediate => "ORI",
            AvrInstructionSet::RelativeCallSubroutine => "RCALL",
            AvrInstructionSet::RelativeJump => "RJMP",
            AvrInstructionSet::SubtractImmediateWithCarrySbi => "SBCI",
            AvrInstructionSet::SubtractImmediate => "SUBI",
            AvrInstructionSet::LoadIndirectWithDisplacementY => "LDD",
            AvrInstructionSet::StoreIndirectWithDisplacementY => "STD",
            AvrInstructionSet::LoadIndirectWithDisplacementZ => "LDD",
            AvrInstructionSet::StoreIndirectWithDisplacementZ => "STD",
            AvrInstructionSet::LoadStoreIndirectWithHightDisplacement => "STD",
        }
    }
}
