fn its_mine_haha(s: String) {
    println!("\"{}\" its mine !", s);
}
fn main() {
    let x: u8 = 22;
    let y = x; // copia el valor de x en y
               // Ojo al Copy Trait
    println!("{}", x); // imprime 22
    println!("{}", y); // imprime 22
    let z: String = String::from("hi");
    let w = z;
    println!("{}", w); // Ojo z se ha movido a w
                       //luego z no se puede usar
    its_mine_haha(w); // w se mueve a la funcion
                      // luego w no se puede usar aqui
}
