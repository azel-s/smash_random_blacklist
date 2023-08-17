#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

use config::UserConfig;
use once_cell::sync::Lazy;

mod config;
mod name;
mod random;

pub static mut RANDOM_WHITELIST_CONFIG: Lazy<UserConfig> =
    Lazy::new(|| UserConfig(std::collections::HashMap::new()));

#[skyline::main(name = "random_whitelist")]
pub fn main() {
    let mythread = std::thread::spawn(|| unsafe {
        match UserConfig::load() {
            Some(whitelist) => {
                *RANDOM_WHITELIST_CONFIG = whitelist;

                name::install();
                random::install();
            }
            None => {
                println!(">[Random-Whitelist]: No config found, plugin will not hook functions.")
            }
        }
    });

    mythread.join().unwrap();
}
