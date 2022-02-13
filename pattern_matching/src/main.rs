use std::env;
use std::process;
fn main() {
    let mut args = env::args().skip(1);
    //std::iter::Skip<env::Args>
    let arg: Option<String> = args.next();
    let some_st: String;
    if let None = arg {
        eprintln!(
            "Me tienes que llamar con un 
    entero sin signo de 8 bits"
        );
        process::exit(3);
    } else {
        some_st = arg.unwrap(); //inicializando el valor
    }
    println!("Tratamos de parsear {}", some_st);
    match some_st.parse::<u8>() {
        Ok(1) => println!("Es uno :D"),
        Ok(2) => println!("Es dos :D"),
        Ok(value) => println!("Es {} :D", value),
        Err(e) => {
            eprintln!("No se puede pasear por {}", e);
            process::exit(2);
        }
    }
} //llave del main
