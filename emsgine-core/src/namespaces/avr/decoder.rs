// generated-code: 2023-05-27T11:50:23.500925

use super::instructions::AvrInstructionSet;
use emsgine_lib::models::bytes::DataWordSized;

macro_rules! extract_bitcoded {

    ($v:expr, $t:tt[ 0x20f ]) => {
        ($v[0] & 0xf | ($v[0] & 0x200) >> 5) as $t
    };

    ($v:expr, $t:tt[ 0x1f0 ]) => {
        (($v[0] & 0x1f0) >> 4) as $t
    };

    ($v:expr, $t:tt[ 0x70 ]) => {
        (($v[0] & 0x70) >> 4) as $t
    };

    ($v:expr, $t:tt[ 0x7 ]) => {
        ($v[0] & 0x7) as $t
    };

    ($v:expr, $t:tt[ 0xf0 ]) => {
        (($v[0] & 0xf0) >> 4) as $t
    };

    ($v:expr, $t:tt[ 0xf ]) => {
        ($v[0] & 0xf) as $t
    };

    ($v:expr, $t:tt[ 0xcf ]) => {
        ($v[0] & 0xf | ($v[0] & 0xc0) >> 2) as $t
    };

    ($v:expr, $t:tt[ 0x30 ]) => {
        (($v[0] & 0x30) >> 4) as $t
    };

    ($v:expr, $t:tt[ 0x1f1ffff ]) => {
        (($v[1] as u32 & 0xffff) >> 16 | $v[0] as u32 & 0x1 | ($v[0] as u32 & 0x1f0) >> 3) as $t
    };

    ($v:expr, $t:tt[ 0xf8 ]) => {
        (($v[0] & 0xf8) >> 3) as $t
    };

    ($v:expr, $t:tt[ 0xffff ]) => {
        (($v[1] as u32 & 0xffff) >> 16) as $t
    };

    ($v:expr, $t:tt[ 0xf0f ]) => {
        ($v[0] & 0xf | ($v[0] & 0xf00) >> 4) as $t
    };

    ($v:expr, $t:tt[ 0x3f8 ]) => {
        (($v[0] & 0x3f8) >> 3) as $t
    };

    ($v:expr, $t:tt[ 0x60f ]) => {
        ($v[0] & 0xf | ($v[0] & 0x600) >> 5) as $t
    };

    ($v:expr, $t:tt[ 0xfff ]) => {
        ($v[0] & 0xfff) as $t
    };

    ($v:expr, $t:tt[ 0xc07 ]) => {
        ($v[0] & 0x7 | ($v[0] & 0xc00) >> 7) as $t
    };

    ($v:expr, $t:tt[ 0x2 ]) => {
        (($v[0] & 0x200) >> 9) as $t
    };

    ($v:expr, $t:tt[ 0x8 ]) => {
        (($v[0] & 0x8) >> 3) as $t
    };
}

pub fn decode<'a>(inst: Vec<u16>) -> Result<(AvrInstructionSet, Vec<(&'a str, DataWordSized)>), u8> {
    return match 0xf0 & inst[0] {
        0x10 => match 0xc & inst[0] {
            0xc => {
                Ok((
                    AvrInstructionSet::AddWithCarry,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x4 => {
                Ok((
                    AvrInstructionSet::Compare,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x0 => {
                Ok((
                    AvrInstructionSet::CompareSkipIfEqual,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x8 => {
                Ok((
                    AvrInstructionSet::SubtractWithoutCarry,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            _ => Err(0u8)
        },
        0x0 => match 0xc & inst[0] {
            0xc => {
                Ok((
                    AvrInstructionSet::Add,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x4 => {
                Ok((
                    AvrInstructionSet::CompareWithCarry,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x0 => match 0x3 & inst[0] {
                0x3 => match 0x80 & inst[0] {
                    0x0 => match 0x8 & inst[0] {
                        0x8 => {
                            Ok((
                                AvrInstructionSet::FractionalMultiplyUnsigned,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                    ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                                )
                            ))
                        },
                        0x0 => {
                            Ok((
                                AvrInstructionSet::MultiplySignedWithUnsigned,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                    ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    0x80 => match 0x8 & inst[0] {
                        0x0 => {
                            Ok((
                                AvrInstructionSet::FractionalMultiplySigned,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                    ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                                )
                            ))
                        },
                        0x8 => {
                            Ok((
                                AvrInstructionSet::FractionalMultiplySignedWithUnsigned,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                    ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    _ => Err(0u8)
                },
                0x1 => {
                    Ok((
                        AvrInstructionSet::CopyRegisterWord,
                        vec!(
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::MultiplySigned,
                        vec!(
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf]))),
                        )
                    ))
                },
                0x0 => {
                    Ok((
                        AvrInstructionSet::NoOperation,
                        vec!(),
                    ))
                },
                _ => Err(0u8)
            },
            0x8 => {
                Ok((
                    AvrInstructionSet::SubtractWithCarry,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            _ => Err(0u8)
        },
        0x90 => match 0xc & inst[0] {
            0x4 => match 0x2 & inst[0] {
                0x2 => match 0x1 & inst[0] {
                    0x0 => {
                        Ok((
                            AvrInstructionSet::AddWithImmediate,
                            vec!(
                                ("imd", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xcf]))),
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x30]))),
                            )
                        ))
                    },
                    0x1 => {
                        Ok((
                            AvrInstructionSet::SubtractImmediateWord,
                            vec!(
                                ("imd", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xcf]))),
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x30]))),
                            )
                        ))
                    },
                    _ => Err(0u8)
                },
                0x0 => match 0xe & inst[0] {
                    0x4 => {
                        Ok((
                            AvrInstructionSet::ArithmeticShiftRight,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x8 => match 0x180 & inst[0] {
                        0x80 => {
                            Ok((
                                AvrInstructionSet::BitClearInSREG,
                                vec!(
                                    ("s", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                )
                            ))
                        },
                        0x180 => match 0x70 & inst[0] {
                            0x10 => {
                                Ok((
                                    AvrInstructionSet::Break,
                                    vec!(),
                                ))
                            },
                            0x50 => {
                                Ok((
                                    AvrInstructionSet::ExtendedLoadProgramMemory,
                                    vec!(),
                                ))
                            },
                            0x40 => {
                                Ok((
                                    AvrInstructionSet::LoadProgramMemory,
                                    vec!(),
                                ))
                            },
                            0x0 => {
                                Ok((
                                    AvrInstructionSet::Sleep,
                                    vec!(),
                                ))
                            },
                            0x60 => {
                                Ok((
                                    AvrInstructionSet::StoreMemoryProgram,
                                    vec!(),
                                ))
                            },
                            0x70 => {
                                Ok((
                                    AvrInstructionSet::StoreMemoryProgramPostIncrementZ,
                                    vec!(),
                                ))
                            },
                            0x20 => {
                                Ok((
                                    AvrInstructionSet::WatchdogReset,
                                    vec!(),
                                ))
                            },
                            _ => Err(0u8)
                        },
                        0x0 => match 0x1 & inst[0] {
                            0x0 => {
                                Ok((
                                    AvrInstructionSet::BitSet,
                                    vec!(
                                        ("s", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x70]))),
                                    )
                                ))
                            },
                            0x1 => match 0x70 & inst[0] {
                                0x10 => {
                                    Ok((
                                        AvrInstructionSet::ExtendedIndirectJump,
                                        vec!(),
                                    ))
                                },
                                0x0 => {
                                    Ok((
                                        AvrInstructionSet::IndirectJump,
                                        vec!(),
                                    ))
                                },
                                _ => Err(0u8)
                            },
                            _ => Err(0u8)
                        },
                        0x1 => match 0x70 & inst[0] {
                            0x10 => match 0x1 & inst[0] {
                                0x1 => {
                                    Ok((
                                        AvrInstructionSet::ExtendedIndirectCallSubroutine,
                                        vec!(),
                                    ))
                                },
                                0x0 => {
                                    Ok((
                                        AvrInstructionSet::ReturnFromInterrupt,
                                        vec!(),
                                    ))
                                },
                                _ => Err(0u8)
                            },
                            0x0 => match 0x1 & inst[0] {
                                0x1 => {
                                    Ok((
                                        AvrInstructionSet::IndirectCallSubroutine,
                                        vec!(),
                                    ))
                                },
                                0x0 => {
                                    Ok((
                                        AvrInstructionSet::ReturnFromSubroutine,
                                        vec!(),
                                    ))
                                },
                                _ => Err(0u8)
                            },
                            _ => Err(0u8)
                        },
                        _ => Err(0u8)
                    },
                    0xe => {
                        if 4 > inst.len() {
                            return Err(4u8)
                        }
                    
                        Ok((
                            AvrInstructionSet::Call,
                            vec!(
                                ("k", DataWordSized::DataSizeDouble(extract_bitcoded!(inst, u32[0x1f1ffff]))),
                            )
                        ))
                    },
                    0x0 => match 0x1 & inst[0] {
                        0x0 => {
                            Ok((
                                AvrInstructionSet::Complement,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        0x1 => {
                            Ok((
                                AvrInstructionSet::ComplementTwo,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    0xa => match 0x1 & inst[0] {
                        0x0 => {
                            Ok((
                                AvrInstructionSet::Decrement,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        0x1 => {
                            Ok((
                                AvrInstructionSet::DataEncryptionStandart,
                                vec!(
                                    ("imd", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    0x2 => match 0x1 & inst[0] {
                        0x1 => {
                            Ok((
                                AvrInstructionSet::Increment,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        0x0 => {
                            Ok((
                                AvrInstructionSet::SwapNibbles,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    0xc => {
                        if 4 > inst.len() {
                            return Err(4u8)
                        }
                    
                        Ok((
                            AvrInstructionSet::Jump,
                            vec!(
                                ("k", DataWordSized::DataSizeDouble(extract_bitcoded!(inst, u32[0x1f1ffff]))),
                            )
                        ))
                    },
                    0x6 => match 0x1 & inst[0] {
                        0x0 => {
                            Ok((
                                AvrInstructionSet::LogicalShiftRight,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        0x1 => {
                            Ok((
                                AvrInstructionSet::RotateRightThroughCarry,
                                vec!(
                                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                )
                            ))
                        },
                        _ => Err(0u8)
                    },
                    _ => Err(0u8)
                },
                _ => Err(0u8)
            },
            0x8 => match 0x3 & inst[0] {
                0x0 => {
                    Ok((
                        AvrInstructionSet::ClearIOBit,
                        vec!(
                            ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf8]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::SetBitIoSpace,
                        vec!(
                            ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf8]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                0x1 => {
                    Ok((
                        AvrInstructionSet::SkipIfIoBitCleared,
                        vec!(
                            ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf8]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                0x3 => {
                    Ok((
                        AvrInstructionSet::SkipIfIoBitSet,
                        vec!(
                            ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf8]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                _ => Err(0u8)
            },
            0x0 => match 0x2 & inst[0] {
                0x0 => match 0xf & inst[0] {
                    0x6 => {
                        Ok((
                            AvrInstructionSet::ExtendedLoadProgramMemoryZ,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x7 => {
                        Ok((
                            AvrInstructionSet::ExtendedLoadProgramMemoryZplus,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xc => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpaceIndexX,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xd => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementX,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xe => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementX,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x9 => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementY,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xa => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementY,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x1 => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePostIncrementZ,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x2 => {
                        Ok((
                            AvrInstructionSet::LoadIndirectFromDataSpacePreDecrementZ,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x0 => {
                        if 4 > inst.len() {
                            return Err(4u8)
                        }
                    
                        Ok((
                            AvrInstructionSet::LoadDirectFromDataSpace,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                ("k", DataWordSized::DataSizeDouble(extract_bitcoded!(inst, u32[0xffff]))),
                            )
                        ))
                    },
                    0x4 => {
                        Ok((
                            AvrInstructionSet::LoadProgramMemoryZ,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x5 => {
                        Ok((
                            AvrInstructionSet::LoadProgramMemoryZplus,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xf => {
                        Ok((
                            AvrInstructionSet::PopRegisterFromStack,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    _ => Err(0u8)
                },
                0x2 => match 0xf & inst[0] {
                    0x6 => {
                        Ok((
                            AvrInstructionSet::LoadAndClear,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x5 => {
                        Ok((
                            AvrInstructionSet::LoadAndSet,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x7 => {
                        Ok((
                            AvrInstructionSet::LoadAndToggle,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xf => {
                        Ok((
                            AvrInstructionSet::PushRegisterToStack,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xc => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpaceIndexX,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xd => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePostIncrementX,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xe => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePreDecrementX,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x9 => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePostIncrementY,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0xa => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePreDecrementY,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x1 => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePostIncrementZ,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x2 => {
                        Ok((
                            AvrInstructionSet::StoreIndirectDataSpacePreDecrementZ,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    0x0 => {
                        if 4 > inst.len() {
                            return Err(4u8)
                        }
                    
                        Ok((
                            AvrInstructionSet::StoreDirectDataSpace,
                            vec!(
                                ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                                ("k", DataWordSized::DataSizeDouble(extract_bitcoded!(inst, u32[0xffff]))),
                            )
                        ))
                    },
                    0x4 => {
                        Ok((
                            AvrInstructionSet::Exchange,
                            vec!(
                                ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            )
                        ))
                    },
                    _ => Err(0u8)
                },
                _ => Err(0u8)
            },
            0xc => {
                Ok((
                    AvrInstructionSet::MultiplyUnsigned,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            _ => Err(0u8)
        },
        0x20 => match 0xc & inst[0] {
            0x0 => {
                Ok((
                    AvrInstructionSet::LogicalAnd,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x4 => {
                Ok((
                    AvrInstructionSet::LogicalExclusiveOr,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0xc => {
                Ok((
                    AvrInstructionSet::CopyRegister,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x8 => {
                Ok((
                    AvrInstructionSet::LogicalOr,
                    vec!(
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x20f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            _ => Err(0u8)
        },
        0x70 => {
            Ok((
                AvrInstructionSet::LogicalAndWithImmediate,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0xf0 => match 0xc & inst[0] {
            0x8 => match 0x2 & inst[0] {
                0x0 => {
                    Ok((
                        AvrInstructionSet::BitLoadInRegisterfromT,
                        vec!(
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::BitStorefromRegister,
                        vec!(
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                _ => Err(0u8)
            },
            0x4 => {
                Ok((
                    AvrInstructionSet::BranchSregBitCleared,
                    vec!(
                        ("k", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x3f8]))),
                        ("s", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                    )
                ))
            },
            0x0 => {
                Ok((
                    AvrInstructionSet::BranchSregBitSet,
                    vec!(
                        ("k", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x3f8]))),
                        ("s", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                    )
                ))
            },
            0xc => match 0x2 & inst[0] {
                0x0 => {
                    Ok((
                        AvrInstructionSet::SkipIfBitCleared,
                        vec!(
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::SkipIfBitSet,
                        vec!(
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                            ("b", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x7]))),
                        )
                    ))
                },
                _ => Err(0u8)
            },
            _ => Err(0u8)
        },
        0x30 => {
            Ok((
                AvrInstructionSet::CompareWithImmediate,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0xb0 => match 0x8 & inst[0] {
            0x0 => {
                Ok((
                    AvrInstructionSet::LoadFromIOSpace,
                    vec!(
                        ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x60f]))),
                        ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            0x8 => {
                Ok((
                    AvrInstructionSet::StoreToIoSpace,
                    vec!(
                        ("rio", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x60f]))),
                        ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    )
                ))
            },
            _ => Err(0u8)
        },
        0xe0 => {
            Ok((
                AvrInstructionSet::LoadImmediate,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0x60 => {
            Ok((
                AvrInstructionSet::LogicalOrWithImmediate,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0xd0 => {
            Ok((
                AvrInstructionSet::RelativeCallSubroutine,
                vec!(
                    ("k", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xfff]))),
                )
            ))
        },
        0xc0 => {
            Ok((
                AvrInstructionSet::RelativeJump,
                vec!(
                    ("k", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xfff]))),
                )
            ))
        },
        0x40 => {
            Ok((
                AvrInstructionSet::SubtractImmediateWithCarrySbi,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0x50 => {
            Ok((
                AvrInstructionSet::SubtractImmediate,
                vec!(
                    ("imd", DataWordSized::DataSizeWord(extract_bitcoded!(inst, u16[0xf0f]))),
                    ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xf0]))),
                )
            ))
        },
        0x80 => match 0x8 & inst[0] {
            0x8 => match 0x2 & inst[0] {
                0x0 => {
                    Ok((
                        AvrInstructionSet::LoadIndirectWithDisplacementY,
                        vec!(
                            ("q", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xc07]))),
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::StoreIndirectWithDisplacementY,
                        vec!(
                            ("q", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xc07]))),
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                        )
                    ))
                },
                _ => Err(0u8)
            },
            0x0 => match 0x2 & inst[0] {
                0x0 => {
                    Ok((
                        AvrInstructionSet::LoadIndirectWithDisplacementZ,
                        vec!(
                            ("q", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xc07]))),
                            ("rds", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                        )
                    ))
                },
                0x2 => {
                    Ok((
                        AvrInstructionSet::StoreIndirectWithDisplacementZ,
                        vec!(
                            ("q", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xc07]))),
                            ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                        )
                    ))
                },
                _ => Err(0u8)
            },
            _ => Err(0u8)
        },
        0xa0 => {
            Ok((
                AvrInstructionSet::LoadStoreIndirectWithHightDisplacement,
                vec!(
                    ("q", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0xc07]))),
                    ("f", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x2]))),
                    ("rsr", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x1f0]))),
                    ("a", DataWordSized::DataSizeByte(extract_bitcoded!(inst, u8[0x8]))),
                )
            ))
        }, // see...,
        _ => Err(0u8)
    };
}