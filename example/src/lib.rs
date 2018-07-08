#![feature(proc_macro)]

extern crate attr;

use attr::repro;

#[repro]
#[magic(foo = "bar")]
pub fn test() {}
