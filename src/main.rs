#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)] // don't mangle the name of the function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function 
    // named `_start` by default
    loop{}
}

/// This function is called a panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //  when a panic occurs, the compiler should invoke this because it has lost 
    //  access to the panic function that was there in the standard library as we 
    //  have disabled it
    // ! means never return. 
    // And to hold it down, we are running an infinite loop
    loop {}
}


// NOTE: There is something called "eh_personality" in Rust
// I am not sure if it is a Rust specific thing but as far as I understand it 
// it marks a function for stack unwinding. Rust runs unwinding to run the 
// destructors of all variables in case of a panic.
//
// Unwinding is complicated. So let's abort in case of panic :)
