#[path = "../../linked_list_second/src/linked_list_second.rs"]
mod linked_list_second;
use crate::linked_list_second::List;
macro_rules! multiply_add {
    ($a:expr , $b:expr, $c:expr , u16) => {
        ($b + $c) * $a
    };
    ($a:expr , $b:expr, $c:expr , &List<u16>) => {
        (&($b + $c)) * $a
    };
}

fn main() {
    let mut list: List<u16> = List::new();
    let mut list_2: List<u16> = List::new();
    list.append(73);
    list_2.append(1729);
    println!("{}", multiply_add!(2, 3, 4, u16));
    println!(
        "{}",
        multiply_add!(2 + 1, &(&list + &list_2), &List::new(), &List<u16>)
    );
}
