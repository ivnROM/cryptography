mod utils;
mod user;

fn main() {
    loop {
        println!("Bienvenido al sistema de autenticación!\n[1] Login\n[2] Register\n[3] Exit");
        let input: u8 = match utils::get_input().parse::<u8>() {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };

        match input {
            1 => (),
            2 => (),
            3 => break,
            _ => continue,
        }
    }

}
