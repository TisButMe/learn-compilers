use std::io::stdin;

fn main() {
    let look_ahead: uint = get_number();
    emit_ln(look_ahead.to_string().as_slice());
}

fn next_char() -> char {
    stdin().read_char().unwrap()
}

fn get_number() -> uint {
    match next_char().to_digit(10) {
        Some(n) => n,
        None    => panic!(expected("Integer"))
    }
}

fn error(s: &str) -> String {
    format!("Error: {}.", s)
}

fn expected(s: &str) -> String {
    error((s.to_string() + " expected".to_string()).as_slice())
}

fn emit(s: &str) {
    print!("\t{}", s);
}

fn emit_ln(s: &str) {
    emit((s.to_string() + "\n".to_string()).as_slice());
}

