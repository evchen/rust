#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]

extern crate rlibc;
extern crate volatile;
extern crate spin;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume()->!{
loop{}
}

mod fib;
#[macro_use]
mod vga_buffer;


#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page
    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Hello again");
    //write!(vga_buffer::WRITER.lock(),", some numbers: {} {}\n", 42,1.337);
    vga_buffer::clear_screen();
    println!("fub {}",fib::fub(233));
    //vga_buffer::print_something();
    loop{}
}






#[lang = "eh_personality"] extern fn eh_personaility(){}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
