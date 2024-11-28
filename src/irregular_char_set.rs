//! A lot of chars only occur some very small number of times. To make processing easier,
//! rather than having tokens for each of these, we stub them out with spaces and have an
//! upfront replacement map at the start of the compresion giving their replacement positions
//! in the file.

pub struct IrregularCharSet {
    replacements: alloc::collections::BTreeMap<char, alloc::vec::Vec<u64>>,
}

impl IrregularCharSet {}
