#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

pub static mut FIGHTER_MANAGER : usize = 0;

mod elight;
mod eflame;
mod switch;

#[skyline::main(name = "based aegis")]
pub fn main() {
    elight::install();
    eflame::install();
    switch::install();
}