#![feature(proc_macro_hygiene)]
extern crate proc_macro;

macro_rules! forward {
    ($test:expr) => {$test}
}

macro_rules! forward_normal_expr {
    ($test:expr) => {forward!($test)}
}

macro_rules! forward_normal_ident {
    ($test:ident) => {forward!($test)}
}

macro_rules! forward_proc_expr {
    ($test:expr) => {proc_macro::forward!($test)}
}

macro_rules! forward_proc_ident {
    ($test:ident) => {proc_macro::forward!($test)}
}

fn main() {
    let a = 1;
    println!("{}", a);
    println!("{}", forward!(a));
    println!("{}", proc_macro::forward!(a));
    println!("{}", forward_normal_expr!(a));
    println!("{}", forward_normal_ident!(a));
    println!("{}", forward_proc_expr!(a));
    println!("{}", forward_proc_ident!(a));
}
