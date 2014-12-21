use std::io::stdin;

fn main() {
    print!("> ");
    let mut look_ahead = next_char();
    
    loop {
        if look_ahead == '.' {
            let op = next_char();
            match op {
                'q' => break,
                _ => {
                    emit_ln(format!("Unknown operation: {}", op).as_slice());
                    look_ahead = next_char();
                }
            }
        } else {
            let val = expression(&mut look_ahead);
            emit_ln(val.to_string().as_slice());
        }
        print!("> ");
        look_ahead = next_char();
    }
}

fn expression(look_ahead: &mut char) -> int {
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
    val
}

fn term(look_ahead: &mut char) -> int {
    let mut val = factor(look_ahead);
    
    *look_ahead = next_char();
    while is_mulop(look_ahead) {
        match *look_ahead {
            '*' => {
                *look_ahead = next_char();
                val *= factor(look_ahead);
            }
            '/' => {
                *look_ahead = next_char();
                val /= factor(look_ahead);
            }
            _ => panic!(expected("mulop"))
        }
        *look_ahead = next_char();
    }
    val
}

fn factor(look_ahead: &mut char) -> int {
    match *look_ahead {
        '(' => {
            *look_ahead = next_char();
            let val = expression(look_ahead);
            if *look_ahead != ')' {panic!(expected(")"));}
            val
        }
        _ => get_number(look_ahead)
    }
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

