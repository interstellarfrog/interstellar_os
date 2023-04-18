// We Cannot Use The Standard Lib As It has OS Specific Functions.
#![no_std]
// A Function Is Called Before The Main Function Which Sets Up The Environment.
// So We Need To OverWrite This As We Do Not Have The OS We Are Coding One.
// And We Do Not Have Access To the Rust runtime and crt0.
#![no_main]

//###################
// CROSS COMPILING
//###################
// My Build
//host: x86_64-pc-windows-msvc
//release: 1.71.0-nightly

// Target Build
// To Avoid Linker Errors We want to Compile Our Code Using --target thumbv7em-none-eabihf.
// This Target Is For An Embedded ARM System Which Does Not Really Matter As All We Need Is A Target That Does Not Have An OS.
// rustup target add thumbv7em-none-eabihf
// cargo build --target thumbv7em-none-eabihf

use core::panic::PanicInfo;

// This function is called on panic.
// The Panic Info Contains Information About The Panic.
// The ! means that this function will never return.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// Dont Mangle The Name Of This Function.
// Extern C means that this function uses the C calling convention.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}



