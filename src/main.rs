#![no_std]
#![no_main]

extern crate libc;

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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    exit_0()
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    exit_1()
}
