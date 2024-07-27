use std::io::{stdin, Read};

pub fn get_input() -> String {
    let mut input: String = String::new();
    stdin().read_to_string(&mut input).expect("Error leyendo el input");
    input.trim()
         .to_string()
}
