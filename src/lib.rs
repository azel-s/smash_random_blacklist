#![feature(concat_idents, proc_macro_hygiene)]
#![allow(unused_macros)]

use std::thread;
use config::UserConfig;
use once_cell::sync::Lazy;

mod config;
mod random;
mod name;

pub static mut RANDOM_ALLOW_CONFIG_DATA: Lazy<UserConfig> = Lazy::new(|| UserConfig(std::collections::HashMap::new()));

#[skyline::main(name = "smashline_custom_random")]
pub fn main() {
    let mythread = std::thread::spawn(|| unsafe {
        match UserConfig::load() {
            Some(something) => {
                unsafe {
                    println!("[Random-Allow] {:?}", something);
                    *RANDOM_ALLOW_CONFIG_DATA = something;

                    random::install();
                    name::install();
                }
            },
            None => {
                println!("[Random-Allow] No config found, plugin will not hook functions.")
            }
        }
    });

    mythread.join().unwrap();
}
