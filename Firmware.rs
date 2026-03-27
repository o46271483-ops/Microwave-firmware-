#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let user_status: u64;
    unsafe {
        asm!("mov {0}, rax", out(reg) user_status);
    }

    if user_status == 0xDEAD {
        unsafe {
            asm!(
                "mov rax, 10",
                "syscall",
                "mov rax, 60",
                "xor rdi, rdi",
                "syscall",
                options(noreturn)
            );
        }
    }

    loop {}
}

#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
