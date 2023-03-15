mod linked_list;
use crate::linked_list::List;
use colored::Colorize;
fn main() {
    let color: u8 = 35;
    let mut list: List<u16> = List::new();
    println!("\x1B[1;32m{}", list); // no hace falta libreria :/
    list.push(32);
    println!("\x1B[1;{}m{}", color, list); // no hace falta libreria :/
    list.push(37);
    list.push(42);
    list.push(1729);
    list.push(73);
    list.push(99);
    println!("{}", format!("{}", list).red().bold());
    //println!("{}", list.pop().unwrap());
    println!("{}", format!("{}", list).yellow().bold());
    println!("{}", format!("{}", list).red().bold());
    println!("{}", format!("{}", list.len()).blue().bold());
    list.pop().unwrap();
    println!("{}", format!("{}", list).red().bold());
    println!("{}", format!("{}", list).yellow().bold());
    println!("\x1B[1;{}m{}", color, list); // no hace falta libreria :/
    println!("{}", format!("{}", list.len()).blue().bold());
}
