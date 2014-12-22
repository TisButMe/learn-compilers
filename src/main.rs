use std::io::stdin;

fn main() {
    print!("> ");
    let mut look_ahead = next_char();
    
    loop {
        if look_ahead == '.' {
            look_ahead = next_char();
            let op = get_name(&mut look_ahead);
            match op.as_slice() {
                "quit" => break,
                _ => {
                    emit_ln(format!("Unknown operation: {}", op).as_slice());
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
    }
    val
}

fn factor(look_ahead: &mut char) -> int {
    match *look_ahead {
        '(' => {
            *look_ahead = next_char();
            let val = expression(look_ahead);
            if *look_ahead != ')' {panic!(expected(")"));}
          *look_ahead = next_char();
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
    let mut c = stdin().read_char().unwrap();

    while c == ' ' {
       c = stdin().read_char().unwrap();
    }

    c
}

fn get_number(look_ahead: &mut char) -> int {
    if !look_ahead.is_digit(10) {panic!("Digit expected, found {}", look_ahead);}
    let mut val = 0;

    while look_ahead.is_digit(10) {
        val = val*10 + look_ahead.to_digit(10).unwrap();
        *look_ahead = next_char();
    }

    val.to_int().unwrap()
}

fn get_name(look_ahead: &mut char) -> String {
    if !look_ahead.is_alphabetic() {panic!("Identifiers must start with a letter")}
    let mut id = String::new();
    
    while look_ahead.is_alphanumeric() {
        id = id + look_ahead.to_string();
        *look_ahead = next_char();
    }

    id
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

