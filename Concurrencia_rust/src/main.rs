#![feature(thread_is_running)]
mod primer;
use crate::primer::*;
use rand::{thread_rng, Rng};
use std::thread::JoinHandle;
fn main() {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(1..=10000);
    let mut threads: Vec<JoinHandle<()>> = vec![];
    for i in 1..=n {
        let time: u64 = rng.gen_range(0..=3) * 1000;
        let t = Th::new(i, time);
        let t = t.start();
        threads.push(t);
    }
    let mut i: usize = 0;
    while i < threads.len() {
        let th = threads.swap_remove(0);
        // Ojo funcion unicamente en Nightly Rust.
        if th.is_running() {
            th.join().unwrap();
        }
        i = i + 1;
    }
    println!("Main Siempre ultimo :D");
}
