use std::io::stdin;

fn main() {
    let mut look_ahead = next_char();
    expression(&mut look_ahead);
}

fn expression(look_ahead: &mut char) {
    let mut val = if is_addop(look_ahead) {
        0
    } else {
        term(look_ahead)
    };

    while is_addop(look_ahead) {
        match *look_ahead {
            '+' => {
                *look_ahead = next_char();
                val += term(look_ahead);
            }
            '-' => {
                *look_ahead = next_char();
                val -= term(look_ahead);
            }
            _ => panic!(expected("addop"))
        }
    }
    emit_ln(val.to_string().as_slice());
}

fn term(look_ahead: &mut char) -> int {
    let mut val = get_number(look_ahead);
    
    *look_ahead = next_char();
    while is_mulop(look_ahead) {
        match *look_ahead {
            '*' => {
                *look_ahead = next_char();
                val *= get_number(look_ahead);
            }
            '/' => {
                *look_ahead = next_char();
                val /= get_number(look_ahead);
            }
            _ => panic!(expected("mulop"))
        }
        *look_ahead = next_char();
    }
    val
}

fn is_addop(c: &char) -> bool {
    ['+', '-'].contains(c)
}

fn is_mulop(c: &char) -> bool {
    ['*', '/'].contains(c)
}

fn next_char() -> char {
    stdin().read_char().unwrap()
}

fn get_number(look_ahead: &char) -> int {
    match look_ahead.to_digit(10) {
        Some(n) => n.to_int().unwrap(),
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

