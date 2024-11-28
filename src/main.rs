#![cfg_attr(not(debug_assertions), no_std)]
#![cfg_attr(not(debug_assertions), no_main)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate libc;

#[cfg(not(debug_assertions))]
#[inline(always)]
fn exit_0() -> ! {
    unsafe {
        core::arch::asm!(
            "mov edi, 0",
            "mov eax, 60",
            "syscall",
            options(nostack, noreturn)
        )
    }
}

#[cfg(not(debug_assertions))]
#[inline(always)]
fn exit_1() -> ! {
    unsafe {
        core::arch::asm!(
            "mov edi, 1",
            "mov eax, 60",
            "syscall",
            options(nostack, noreturn)
        )
    }
}

#[cfg(not(debug_assertions))]
mod use_malloc {
    // On nostd link against malloc
    use alloc::alloc::*;

    /// The global allocator type.
    #[derive(Default)]
    pub struct Allocator;

    unsafe impl core::alloc::GlobalAlloc for Allocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            libc::malloc(layout.size()) as *mut u8
        }
        unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
            libc::free(ptr as *mut libc::c_void);
        }
    }

    /// If there is an out of memory error, just panic.
    #[alloc_error_handler]
    fn my_allocator_error(_layout: core::alloc::Layout) -> ! {
        super::exit_1()
    }

    /// The static global allocator.
    #[global_allocator]
    static GLOBAL_ALLOCATOR: Allocator = Allocator;
}

#[cfg(debug_assertions)]
pub fn main() {
    let args: Vec<_> = std::env::args()
        .map(|arg| std::ffi::CString::new(arg).unwrap())
        .collect();
    let arg_refs = args.iter().map(|s| s.as_ptr()).collect::<Vec<_>>();

    nscompress::run(args.len() as libc::c_int, arg_refs.as_ptr());
}

#[cfg(not(debug_assertions))]
#[no_mangle]
pub extern "C" fn main(arg_c: libc::c_int, arg_v: *const *const libc::c_char) -> ! {
    nscompress::run(arg_c, arg_v);

    exit_0()
}

#[cfg(not(debug_assertions))]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    exit_1()
}
