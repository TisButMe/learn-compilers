use std::io::stdin;

fn main() {
    let mut look_ahead = ' ';
    expression(&mut look_ahead);
}

fn expression(look_ahead: &mut char) {
    term(look_ahead);
    while vec!('+', '-').contains(look_ahead) {
        emit_ln("PUSH RAX".to_string());
        match *look_ahead {
            '+' => add(look_ahead),
            '-' => sub(look_ahead),
            _   => panic!(expected("addop".to_string()))
        }
    }
}

fn term(look_ahead: &mut char) {
    factor();
    *look_ahead = next_char();
    while vec!('*', '/').contains(look_ahead) {
        emit_ln("PUSH RAX".to_string());
        match *look_ahead {
            '*' => mul(),
            '/' => div(),
            _   => panic!(expected("mulop".to_string()))
        }
        *look_ahead = next_char();
    }
}

fn add(look_ahead: &mut char) {
    term(look_ahead);
    emit_ln("POP RCX".to_string());
    emit_ln("ADD RAX, RCX".to_string());
}

fn sub(look_ahead: &mut char) {
    term(look_ahead);
    emit_ln("POP RCX".to_string());
    emit_ln("SUB RAX, RCX".to_string());
    emit_ln("NEG RAX".to_string());
}

fn mul() {
    factor();
    emit_ln("POP RCX".to_string());
    emit_ln("IMUL RCX".to_string());
}

fn div() {
    factor();
    emit_ln("MOV RCX, RAX".to_string());
    emit_ln("POP RAX".to_string());
    emit_ln("XOR RDX, RDX".to_string());
    emit_ln("IDIV RCX".to_string());
}

fn factor() {
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

