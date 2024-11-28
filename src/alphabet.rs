//! Reads a file and keeps a map of all present utf-8 characters.
//! Also converts the file to a static-stride list of codepoints in memory.

use core::num::NonZero;

use crate::std;

const CONT_MASK: u8 = 0b0011_1111;

#[inline]
const fn utf8_first_byte(byte: u8, width: u32) -> u32 {
    (byte & (0x7F >> width)) as u32
}

/// Returns the value of `ch` updated with continuation byte `byte`.
#[inline]
const fn utf8_acc_cont_byte(ch: u32, byte: u8) -> u32 {
    (ch << 6) | (byte & CONT_MASK) as u32
}

// TODO: executable size would be smaller if this operated over slices rather than an iterator.
fn next_code_point<'a, I: Iterator<Item = &'a u8>>(bytes: &mut I) -> Option<u32> {
    // Decode UTF-8
    let x = *bytes.next()?;
    if x < 128 {
        return Some(x as u32);
    }

    // Multibyte case follows
    // Decode from a byte combination out of: [[[x y] z] w]
    // NOTE: Performance is sensitive to the exact formulation here
    let init = utf8_first_byte(x, 2);
    // SAFETY: `bytes` produces an UTF-8-like string,
    // so the iterator must produce a value here.
    let y = *bytes.next()?;
    let mut ch = utf8_acc_cont_byte(init, y);
    if x >= 0xE0 {
        // [[x y z] w] case
        // 5th bit in 0xE0 .. 0xEF is always clear, so `init` is still valid
        // SAFETY: `bytes` produces an UTF-8-like string,
        // so the iterator must produce a value here.
        let z = *bytes.next()?;
        let y_z = utf8_acc_cont_byte((y & CONT_MASK) as u32, z);
        ch = init << 12 | y_z;
        if x >= 0xF0 {
            // [x y z w] case
            // use only the lower 3 bits of `init`
            // SAFETY: `bytes` produces an UTF-8-like string,
            // so the iterator must produce a value here.
            let w = *bytes.next()?;
            ch = (init & 7) << 18 | utf8_acc_cont_byte(y_z, w);
        }
    }

    Some(ch)
}

#[derive(Clone, Copy)]
pub struct CharIndex(NonZero<u16>);

pub struct Codepoints(alloc::vec::Vec<CharIndex>);

impl Codepoints {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn ith(&self, i: usize) -> CharIndex {
        self.0[i]
    }

    pub fn decode(&self, i: usize, alphabet: &Alphabet) -> char {
        alphabet.lookup_decoding(self.0[i])
    }

    pub fn occurrences(&self, alphabet: &Alphabet) -> alloc::vec::Vec<(char, i32)> {
        let mut occurrences = alloc::vec![('\0', 0); alphabet.count() as usize];
        for i in 0..alphabet.count() {
            let idx = CharIndex(unsafe { NonZero::new_unchecked(i + 1) });
            occurrences[i as usize].0 = alphabet.lookup_decoding(idx);
        }
        for i in 0..self.len() {
            occurrences[self.ith(i).0.get() as usize - 1].1 += 1;
        }

        occurrences.sort_by_key(|(_, count)| *count);

        occurrences
    }
}

struct AlphabetInner {
    // Dense mappings from observed unicodes to indices.
    // Each of these maps takes about 1MB in Memory.
    chars_to_indices: [Option<CharIndex>; char::MAX as usize],
    indices_to_chars: [char; u16::MAX as usize],
    count: u16,
}

impl Default for AlphabetInner {
    fn default() -> Self {
        Self {
            chars_to_indices: [None; char::MAX as usize],
            indices_to_chars: ['\0'; u16::MAX as usize],
            count: 0,
        }
    }
}

pub struct Alphabet {
    inner: alloc::boxed::Box<AlphabetInner>,
}

impl Alphabet {
    pub fn from_file(file: &std::file::File) -> (Self, Codepoints) {
        let mut alphabet = alloc::boxed::Box::new(AlphabetInner::default());
        let mut codepoints = Codepoints(alloc::vec::Vec::new());

        let mut trailing = [0; 4];
        let mut trailing_count = 0;

        let mut buf = [0; 4096];
        loop {
            let n = match file.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => panic!("{e:?}"),
            };

            let mut byte_iter = trailing[..trailing_count].iter().chain(buf[..n].iter());
            unsafe {
                while let Some(next_char) = next_code_point(&mut byte_iter) {
                    let char = char::from_u32_unchecked(next_char);

                    let index = &mut alphabet.chars_to_indices[char as usize];
                    let index = match index {
                        Some(index) => *index,
                        None => {
                            alphabet.count += 1;
                            let new_index = NonZero::new_unchecked(alphabet.count);
                            *index = Some(CharIndex(new_index));
                            alphabet.indices_to_chars[new_index.get() as usize - 1] = char;
                            CharIndex(new_index)
                        }
                    };

                    codepoints.0.push(index);
                }
            }

            let mut tmp_trailing = [0; 4];
            trailing_count = 0;
            while let Some(b) = byte_iter.next() {
                tmp_trailing[trailing_count] = *b;
                trailing_count += 1;
            }
            trailing = tmp_trailing;
        }

        (Self { inner: alphabet }, codepoints)
    }

    pub fn count(&self) -> u16 {
        self.inner.count
    }

    pub fn lookup_coding(&self, c: char) -> Option<CharIndex> {
        self.inner.chars_to_indices[c as usize]
    }

    pub fn lookup_decoding(&self, index: CharIndex) -> char {
        self.inner.indices_to_chars[index.0.get() as usize - 1]
    }

    #[allow(unused)]
    pub fn chars(&self) -> &[char] {
        &self.inner.indices_to_chars[..self.inner.count as usize]
    }
}
