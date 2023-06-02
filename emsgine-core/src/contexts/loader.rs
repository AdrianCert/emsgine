use std::fs;
use std::rc::Rc;

use ihex::Reader as HexReader;
use ihex::Record as HexRecord;

use super::MemoryContext;

pub fn load_memory_from_hexfile<'a>(
    filepath: &'a str,
    context: Rc<dyn MemoryContext<Output = u8>>,
) -> bool {
    if let Ok(content) = fs::read_to_string(filepath) {
        HexReader::new(content.as_str()).for_each(|rec| {
            if let Ok(rec) = rec {
                if let HexRecord::Data { offset, value } = rec {
                    context.write_bytes(offset.into(), value)
                }
            }
        });

        return true;
    }

    false
}
