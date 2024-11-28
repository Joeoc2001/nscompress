//! Tokenize the input into a stream of tokens using a Huffman tree.

use slab::Slab;

use crate::std;

pub struct Grammar {
    mem: Slab<GrammarNode>,
}

pub struct GrammarNode {}

impl Grammar {
    pub fn from_file(file: &std::file::File, alphabet: &crate::alphabet::Alphabet) -> Self {
        let mut mem = Slab::new();

        Self { mem }
    }
}
