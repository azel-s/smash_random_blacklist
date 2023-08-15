#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

mod random;
mod name;

#[skyline::main(name = "smashline_custom_random")]
pub fn main() {
    // random::install();
    name::install();
}
