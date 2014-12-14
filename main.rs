use std::io::stdin;

fn main() {
    expression();
}

fn expression() {
    term();
    let mut c = next_char();
    while vec!('+', '-').contains(&c) {
        emit_ln("PUSH RAX".to_string());
        match c {
            '+' => add(),
            '-' => sub(),
            _   => panic!(expected("addop".to_string()))
        }
        c = next_char();
    }
}

fn add() {
    term();
    emit_ln("POP RCX".to_string());
    emit_ln("ADD RAX, RCX".to_string());
}

fn sub() {
    term();
    emit_ln("POP RCX".to_string());
    emit_ln("SUB RAX, RCX".to_string());
    emit_ln("NEG RAX".to_string());
}

fn term() {
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

