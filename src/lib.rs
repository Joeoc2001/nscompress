#![no_std]

mod alphabet;
mod grammar;
mod irregular_char_set;
mod std;

extern crate alloc;

#[cfg(feature = "stdout")]
macro_rules! println {
    ($($args:tt)*) => {
        ::libc_print::libc_println!( $($args)* )
    };
}

#[cfg(not(feature = "stdout"))]
macro_rules! println {
    ($($args:tt)*) => {};
}

use println;

pub fn run(arg_c: libc::c_int, arg_v: *const *const libc::c_char) {
    let args = unsafe { core::slice::from_raw_parts(arg_v, arg_c as usize) };

    let Some(arg1) = args.get(1) else {
        println!("Expected at least one argument");
        panic!();
    };
    let arg1 = unsafe { core::ffi::CStr::from_ptr(*arg1) };

    let mut file_arg = arg1;
    let should_decompress = arg1.to_bytes() == b"-d";
    if should_decompress {
        let Some(arg2) = args.get(2) else {
            println!("Expected a path to decompress");
            panic!();
        };
        let arg2 = unsafe { core::ffi::CStr::from_ptr(*arg2) };
        file_arg = arg2;
    }

    let Ok(input) = std::file::File::open(file_arg, std::file::FileOpen::Read) else {
        println!("Failed to open file");
        panic!();
    };

    if should_decompress {
        decompress(input);
    } else {
        compress(input);
    }
}

fn compress(file: std::file::File) {
    let (alphabet, codepoints) = alphabet::Alphabet::from_file(&file);
    println!("Found {} total unique chars", alphabet.count());
    println!("Char occurrences: {:#?}", codepoints.occurrences(&alphabet));
}

fn decompress(file: std::file::File) {}
