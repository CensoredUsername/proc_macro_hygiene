#![feature(proc_macro_hygiene)]
extern crate proc_macro;

macro_rules! forward {
    ($test:expr) => {proc_macro::forward!($test)}
}

fn main() {
    let a = 1;
    println!("{}", a);
    println!("{}", proc_macro::forward!(a));
    println!("{}", forward!(a));
}
