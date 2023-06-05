// use std::cell::RefCell;
// use std::collections::HashMap;

// use emsgine_lib::models::bytes::EndianByteOrdering;
// use emsgine_core::cpu::CpuState;
// use emsgine_core::actions::InstructionPull;
// use emsgine_core::memory::{MemoryDevice, RegistersPage};

fn test_vec() {
    let mut mem: Vec<u8> = vec![0; 34];
    mem[2] = 12;
    mem[0] = 11;
    print!("{:?}", mem);
}

// fn simulation() {
//     let mut cpu = CpuState {
//         memory: MemoryDevice::new(100),
//         register_page: RegistersPage::<u8> {
//             registers: RefCell::new(vec![0;32]),
//         },
//         internals: HashMap::from([
//             ("PC", 0),
//         ])
//     };

//     let mut isp = InstructionPull {
//         wordsize: 2,
//         endianness: EndianByteOrdering::LittleEndian
//     };

//     isp.run(cpu);

// }

fn main() {
    test_vec();
    // simulation();
    // let mut map = HashMap::new();
    // map.insert("PC", "abc");
    // map.insert("PCR", "abc");
    // println!("{:?}", map["PC"]);
    // println!("{:?}", map["PC"]);
    // println!("{:?}", map["PC"]);
}
