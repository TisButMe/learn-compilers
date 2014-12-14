use std::io::stdin;

fn main() {
    expression();
}

fn expression() {
    emit_ln("MOV RAX, ".to_string() + get_number().to_string())
}

fn next_char() -> char {
    stdin().read_char().unwrap()
}

fn get_number() -> uint {
    match next_char().to_digit(10) {
        Some(n) => n,
        None    => panic!(expected("Integer".to_string()))
    }
}

fn error(s: String) -> String {
    format!("Error: {}.", s)
}

fn expected(s: String) -> String {
    error(s + " expected".to_string())
}

fn emit(s: String) {
    print!("\t{}", s);
}

fn emit_ln(s: String) {
    emit(s + "\n".to_string());
}

