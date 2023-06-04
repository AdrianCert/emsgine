// generated-code: 2023-06-04T23:12:57.029690

use emsgine_lib::models::instructionset::InstructionNamespace;
use emsgine_lib::models::instructionset::MnemonicInstruction;
use emsgine_lib::models::instructionset::FormatInstruction;
use emsgine_lib::models::bytes::DataWordSized;
use emsgine_lib::lookup::Lookup;
use emsgine_lib::lookup::safe_lookup;

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
    PushRegisterToStack, //PUSH Rd
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
            AvrInstructionSet::Invalid => "???"
        }
    }
}

impl FormatInstruction for AvrInstructionSet {
    fn formatstr<'a>(&self) -> &'a str {
        match self {
            AvrInstructionSet::AddWithCarry => "ADC R{},R{}",
            AvrInstructionSet::Compare => "CP R{},R{}",
            AvrInstructionSet::CompareSkipIfEqual => "CPSE R{},R{}",
            AvrInstructionSet::SubtractWithoutCarry => "SUB R{},R{}",
            AvrInstructionSet::Add => "ADD R{},R{}",
            AvrInstructionSet::CompareWithCarry => "CPC R{},R{}",
            AvrInstructionSet::FractionalMultiplyUnsigned => "FMUL R{},R{}",
            AvrInstructionSet::MultiplySignedWithUnsigned => "MULSU R{},R{}",
            AvrInstructionSet::FractionalMultiplySigned => "FMULS R{},R{}",
            AvrInstructionSet::FractionalMultiplySignedWithUnsigned => "FMULSU R{},R{}",
            AvrInstructionSet::CopyRegisterWord => "MOVW R{}+1:R{},R{}+1:R{}",
            AvrInstructionSet::MultiplySigned => "MULS R{},R{}",
            AvrInstructionSet::NoOperation => "NOP",
            AvrInstructionSet::SubtractWithCarry => "SBC R{},R{}",
            AvrInstructionSet::AddWithImmediate => "ADIW R{}+1:R{},{}",
            AvrInstructionSet::SubtractImmediateWord => "SBIW R{}+1:R{},{}",
            AvrInstructionSet::ArithmeticShiftRight => "ASR R{}",
            AvrInstructionSet::BitClearInSREG => "BCLR {}",
            AvrInstructionSet::Break => "BREAK",
            AvrInstructionSet::ExtendedLoadProgramMemory => "ELPM",
            AvrInstructionSet::LoadProgramMemory => "LPM",
            AvrInstructionSet::Sleep => "SLEEP",
            AvrInstructionSet::StoreMemoryProgram => "SPM",
            AvrInstructionSet::StoreMemoryProgramPostIncrementZ => "SPM Z+",
            AvrInstructionSet::WatchdogReset => "WDR",
            AvrInstructionSet::BitSet => "BSET {}",
            AvrInstructionSet::ExtendedIndirectJump => "EIJMP",
            AvrInstructionSet::IndirectJump => "IJMP",
            AvrInstructionSet::ExtendedIndirectCallSubroutine => "EICALL",
            AvrInstructionSet::ReturnFromInterrupt => "RETI",
            AvrInstructionSet::IndirectCallSubroutine => "ICALL",
            AvrInstructionSet::ReturnFromSubroutine => "RET",
            AvrInstructionSet::Call => "CALL {}",
            AvrInstructionSet::Complement => "COM R{}",
            AvrInstructionSet::ComplementTwo => "NEG R{}",
            AvrInstructionSet::Decrement => "DEC R{}",
            AvrInstructionSet::DataEncryptionStandart => "DES {}",
            AvrInstructionSet::Increment => "INC R{}",
            AvrInstructionSet::SwapNibbles => "SWAP R{}",
            AvrInstructionSet::Jump => "JMP {}",
            AvrInstructionSet::LogicalShiftRight => "LSR R{}",
            AvrInstructionSet::RotateRightThroughCarry => "ROR R{}",
            AvrInstructionSet::ClearIOBit => "CBI {},{}",
            AvrInstructionSet::SetBitIoSpace => "SBI {},{}",
            AvrInstructionSet::SkipIfIoBitCleared => "SBIC {},{}",
            AvrInstructionSet::SkipIfIoBitSet => "SBIS {},{}",
            AvrInstructionSet::ExtendedLoadProgramMemoryZ => "ELPM R{}, Z",
            AvrInstructionSet::ExtendedLoadProgramMemoryZplus => "ELPM R{}, Z+",
            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX => "LD R{}, X",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX => "LD R{}, X+",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX => "LD R{}, -X",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY => "LD R{}, Y+",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY => "LD R{}, -Y",
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ => "LD R{}, Z+",
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ => "LD R{}, -Z",
            AvrInstructionSet::LoadDirectFromDataSpace => "LDS R{},{}",
            AvrInstructionSet::LoadProgramMemoryZ => "LPM R{}, Z",
            AvrInstructionSet::LoadProgramMemoryZplus => "LPM R{}, Z+",
            AvrInstructionSet::PopRegisterFromStack => "POP R{}",
            AvrInstructionSet::LoadAndClear => "LAC Z,Rd",
            AvrInstructionSet::LoadAndSet => "LAS Z,Rd",
            AvrInstructionSet::LoadAndToggle => "LAT Z,Rd",
            AvrInstructionSet::PushRegisterToStack => "PUSH R{}",
            AvrInstructionSet::StoreIndirectDataSpaceIndexX => "ST X, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX => "ST X+, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX => "ST -X, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY => "ST Y+, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY => "ST -Y, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ => "ST Z+, R{}",
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ => "ST -Z, R{}",
            AvrInstructionSet::StoreDirectDataSpace => "STS {},Rr",
            AvrInstructionSet::Exchange => "XCH Z,Rd",
            AvrInstructionSet::MultiplyUnsigned => "MUL R{},R{}",
            AvrInstructionSet::LogicalAnd => "AND R{},R{}",
            AvrInstructionSet::LogicalExclusiveOr => "EOR R{},R{}",
            AvrInstructionSet::CopyRegister => "MOV R{},R{}",
            AvrInstructionSet::LogicalOr => "OR R{},R{}",
            AvrInstructionSet::LogicalAndWithImmediate => "ANDI R{},{}",
            AvrInstructionSet::BitLoadInRegisterfromT => "BLD R{},{}",
            AvrInstructionSet::BitStorefromRegister => "BST R{},{}",
            AvrInstructionSet::BranchSregBitCleared => "BRBC {},{}",
            AvrInstructionSet::BranchSregBitSet => "BRBS {},{}",
            AvrInstructionSet::SkipIfBitCleared => "SBRC R{},{}",
            AvrInstructionSet::SkipIfBitSet => "SBRS R{},{}",
            AvrInstructionSet::CompareWithImmediate => "CPI R{},{}",
            AvrInstructionSet::LoadFromIOSpace => "IN R{},{}",
            AvrInstructionSet::StoreToIoSpace => "OUT {},R{}",
            AvrInstructionSet::LoadImmediate => "LDI R{},{}",
            AvrInstructionSet::LogicalOrWithImmediate => "ORI R{},{}",
            AvrInstructionSet::RelativeCallSubroutine => "RCALL {}",
            AvrInstructionSet::RelativeJump => "RJMP {}",
            AvrInstructionSet::SubtractImmediateWithCarrySbi => "SBCI R{},{}",
            AvrInstructionSet::SubtractImmediate => "SUBI R{},{}",
            AvrInstructionSet::LoadIndirectWithDisplacementY => "LDD R{}, Y+{}",
            AvrInstructionSet::StoreIndirectWithDisplacementY => "STD Y+{}, R{}",
            AvrInstructionSet::LoadIndirectWithDisplacementZ => "LDD R{}, Z+{}",
            AvrInstructionSet::StoreIndirectWithDisplacementZ => "STD Z+{}, R{}",
            AvrInstructionSet::LoadStoreIndirectWithHightDisplacement => "STD Y+{}, R{}",
            AvrInstructionSet::Invalid => "???"
        }
    }

    fn format<'a>(&self, ltbl: &'a dyn Lookup<&str, DataWordSized>) -> String {
        match self {
            AvrInstructionSet::AddWithCarry => format!("ADC R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::Compare => format!("CP R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::CompareSkipIfEqual => format!("CPSE R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::SubtractWithoutCarry => format!("SUB R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::Add => format!("ADD R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::CompareWithCarry => format!("CPC R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::FractionalMultiplyUnsigned => format!("FMUL R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::MultiplySignedWithUnsigned => format!("MULSU R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::FractionalMultiplySigned => format!("FMULS R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::FractionalMultiplySignedWithUnsigned => format!("FMULSU R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::CopyRegisterWord => format!("MOVW R{}+1:R{},R{}+1:R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::MultiplySigned => format!("MULS R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::NoOperation =>format!("NOP"),
            AvrInstructionSet::SubtractWithCarry => format!("SBC R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::AddWithImmediate => format!("ADIW R{}+1:R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::SubtractImmediateWord => format!("SBIW R{}+1:R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::ArithmeticShiftRight => format!("ASR R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::BitClearInSREG => format!("BCLR {}", safe_lookup![ltbl, "s"]),
            AvrInstructionSet::Break =>format!("BREAK"),
            AvrInstructionSet::ExtendedLoadProgramMemory =>format!("ELPM"),
            AvrInstructionSet::LoadProgramMemory =>format!("LPM"),
            AvrInstructionSet::Sleep =>format!("SLEEP"),
            AvrInstructionSet::StoreMemoryProgram =>format!("SPM"),
            AvrInstructionSet::StoreMemoryProgramPostIncrementZ =>format!("SPM Z+"),
            AvrInstructionSet::WatchdogReset =>format!("WDR"),
            AvrInstructionSet::BitSet => format!("BSET {}", safe_lookup![ltbl, "s"]),
            AvrInstructionSet::ExtendedIndirectJump =>format!("EIJMP"),
            AvrInstructionSet::IndirectJump =>format!("IJMP"),
            AvrInstructionSet::ExtendedIndirectCallSubroutine =>format!("EICALL"),
            AvrInstructionSet::ReturnFromInterrupt =>format!("RETI"),
            AvrInstructionSet::IndirectCallSubroutine =>format!("ICALL"),
            AvrInstructionSet::ReturnFromSubroutine =>format!("RET"),
            AvrInstructionSet::Call => format!("CALL {}", safe_lookup![ltbl, "k"]),
            AvrInstructionSet::Complement => format!("COM R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::ComplementTwo => format!("NEG R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::Decrement => format!("DEC R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::DataEncryptionStandart => format!("DES {}", safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::Increment => format!("INC R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::SwapNibbles => format!("SWAP R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::Jump => format!("JMP {}", safe_lookup![ltbl, "k"]),
            AvrInstructionSet::LogicalShiftRight => format!("LSR R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::RotateRightThroughCarry => format!("ROR R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::ClearIOBit => format!("CBI {},{}", safe_lookup![ltbl, "rio"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::SetBitIoSpace => format!("SBI {},{}", safe_lookup![ltbl, "rio"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::SkipIfIoBitCleared => format!("SBIC {},{}", safe_lookup![ltbl, "rio"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::SkipIfIoBitSet => format!("SBIS {},{}", safe_lookup![ltbl, "rio"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::ExtendedLoadProgramMemoryZ => format!("ELPM R{}, Z", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::ExtendedLoadProgramMemoryZplus => format!("ELPM R{}, Z+", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX => format!("LD R{}, X", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX => format!("LD R{}, X+", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX => format!("LD R{}, -X", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY => format!("LD R{}, Y+", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY => format!("LD R{}, -Y", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ => format!("LD R{}, Z+", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ => format!("LD R{}, -Z", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadDirectFromDataSpace => format!("LDS R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "k"]),
            AvrInstructionSet::LoadProgramMemoryZ => format!("LPM R{}, Z", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadProgramMemoryZplus => format!("LPM R{}, Z+", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::PopRegisterFromStack => format!("POP R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::LoadAndClear =>format!("LAC Z,Rd"),
            AvrInstructionSet::LoadAndSet =>format!("LAS Z,Rd"),
            AvrInstructionSet::LoadAndToggle =>format!("LAT Z,Rd"),
            AvrInstructionSet::PushRegisterToStack => format!("PUSH R{}", safe_lookup![ltbl, "rds"]),
            AvrInstructionSet::StoreIndirectDataSpaceIndexX => format!("ST X, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX => format!("ST X+, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX => format!("ST -X, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY => format!("ST Y+, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY => format!("ST -Y, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ => format!("ST Z+, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ => format!("ST -Z, R{}", safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::StoreDirectDataSpace => format!("STS {},Rr", safe_lookup![ltbl, "k"]),
            AvrInstructionSet::Exchange =>format!("XCH Z,Rd"),
            AvrInstructionSet::MultiplyUnsigned => format!("MUL R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LogicalAnd => format!("AND R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LogicalExclusiveOr => format!("EOR R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::CopyRegister => format!("MOV R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LogicalOr => format!("OR R{},R{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LogicalAndWithImmediate => format!("ANDI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::BitLoadInRegisterfromT => format!("BLD R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::BitStorefromRegister => format!("BST R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::BranchSregBitCleared => format!("BRBC {},{}", safe_lookup![ltbl, "s"], safe_lookup![ltbl, "k"]),
            AvrInstructionSet::BranchSregBitSet => format!("BRBS {},{}", safe_lookup![ltbl, "s"], safe_lookup![ltbl, "k"]),
            AvrInstructionSet::SkipIfBitCleared => format!("SBRC R{},{}", safe_lookup![ltbl, "rsr"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::SkipIfBitSet => format!("SBRS R{},{}", safe_lookup![ltbl, "rsr"], safe_lookup![ltbl, "b"]),
            AvrInstructionSet::CompareWithImmediate => format!("CPI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::LoadFromIOSpace => format!("IN R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "rio"]),
            AvrInstructionSet::StoreToIoSpace => format!("OUT {},R{}", safe_lookup![ltbl, "rio"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LoadImmediate => format!("LDI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::LogicalOrWithImmediate => format!("ORI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::RelativeCallSubroutine => format!("RCALL {}", safe_lookup![ltbl, "k"]),
            AvrInstructionSet::RelativeJump => format!("RJMP {}", safe_lookup![ltbl, "k"]),
            AvrInstructionSet::SubtractImmediateWithCarrySbi => format!("SBCI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::SubtractImmediate => format!("SUBI R{},{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "imd"]),
            AvrInstructionSet::LoadIndirectWithDisplacementY => format!("LDD R{}, Y+{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "q"]),
            AvrInstructionSet::StoreIndirectWithDisplacementY => format!("STD Y+{}, R{}", safe_lookup![ltbl, "q"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LoadIndirectWithDisplacementZ => format!("LDD R{}, Z+{}", safe_lookup![ltbl, "rds"], safe_lookup![ltbl, "q"]),
            AvrInstructionSet::StoreIndirectWithDisplacementZ => format!("STD Z+{}, R{}", safe_lookup![ltbl, "q"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::LoadStoreIndirectWithHightDisplacement => format!("STD Y+{}, R{}", safe_lookup![ltbl, "q"], safe_lookup![ltbl, "rsr"]),
            AvrInstructionSet::Invalid => format!("???")
        }
    }
}
