mod linked_list_second;
use crate::linked_list_second::List;
fn main() {
    let mut list: List<u16> = List::new();
    let mut list_2: List<u16> = List::new();
    let color_3: u8 = 32;
    let color_2: u8 = 33;
    let color_1: u8 = 35;
    list.append(453);
    list.push(38);
    list.append(67);
    list.append(73);
    list.append(1729);
    list_2.append(17);
    list_2.append(29);
    list_2.append(6174);
    println!("list : \x1B[1;{}m{} \x1B[0m", color_1, list);
    list.pop();
    println!("list : \x1B[1;{}m{} \x1B[0m", color_1, list);
    println!("list_2 : \x1B[1;{}m{} \x1B[0m", color_2, list_2);
    let list_3 = &list + &list_2;
    println!(
        "\x1B[1;{}m{} \x1B[1;37m + \x1B[1;{}m{}  \x1B[1;37m =  \x1B[7m\x1B[1;{}m{}\x1B[0m",
        color_1, list, color_2, list_2, color_3, list_3
    );
    let list_4 = &list_2 * 3;
    println!(
        "\x1B[1;{}m{} \x1B[1;37m * \x1B[1;{}m{}  \x1B[1;37m =  \x1B[7m\x1B[1;{}m{}\x1B[0m",
        color_2, list_2, color_1, 3, color_3, list_4
    );
    let list_5 = 2 * &list_2;
    println!(
        "\x1B[1;{}m{} \x1B[1;37m * \x1B[1;{}m{}  \x1B[1;37m =  \x1B[7m\x1B[1;{}m{}\x1B[0m",
        color_1, 2, color_2, list_2, color_3, list_5
    );
}
