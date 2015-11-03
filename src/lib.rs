#![feature(no_std)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(core_str_ext)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("Wubba");
    println!("Lubba");
    println!("Dub");
    println!("Duuuuub!!!");
    loop{}
}

 

#[cfg(not(test))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {loop{}}
