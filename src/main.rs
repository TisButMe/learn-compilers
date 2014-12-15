use std::io::stdin;

fn main() {
  let mut look_ahead = ' ';
  emit_ln("section .text".to_string());
  emit_ln("global _start".to_string());
  emit_ln("_start:".to_string());
  emit_ln(";; start of code gen".to_string());
  expression(&mut look_ahead);
  emit_ln("\n\t;; end of code gen\n\tMOV RAX, 60".to_string());
  emit_ln("XOR RDI, RDI".to_string());
  emit_ln("SYSCALL".to_string());
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
  factor(look_ahead);
  while vec!('*', '/').contains(look_ahead) {
    emit_ln("PUSH RAX".to_string());
    match *look_ahead {
      '*' => mul(look_ahead),
      '/' => div(look_ahead),
      _   => panic!(expected("mulop".to_string()))
    }
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

fn mul(look_ahead: &mut char) {
  factor(look_ahead);
  emit_ln("POP RCX".to_string());
  emit_ln("IMUL RCX".to_string());
}

fn div(look_ahead: &mut char) {
  factor(look_ahead);
  emit_ln("MOV RCX, RAX".to_string());
  emit_ln("POP RAX".to_string());
  emit_ln("XOR RDX, RDX".to_string());
  emit_ln("IDIV RCX".to_string());
}

fn factor(look_ahead: &mut char) {
  *look_ahead = next_char();

  match *look_ahead {
    '(' => {
      expression(look_ahead);

      if *look_ahead != ')' {
        panic!(expected("delimiter: )".to_string()));
      }

      *look_ahead = next_char();
    },
    '+'|'-'                => emit_ln("XOR RAX, RAX".to_string()),
    x if x.is_alphabetic() => ident(look_ahead),
    _                      => {
      emit_ln("MOV RAX, ".to_string() + get_number(look_ahead).to_string());
      *look_ahead = next_char();
    }
  }
}

fn next_char() -> char {
  stdin().read_char().unwrap()
}

fn get_number(look_ahead: &char) -> uint {
  match look_ahead.to_digit(10) {
    Some(n) => n,
    None    => panic!(expected("Integer".to_string()))
  }
}

fn get_name(look_ahead: &mut char) -> String {
  look_ahead.to_string()
}

fn ident(look_ahead: &mut char) {
  let name = get_name(look_ahead);
  *look_ahead = next_char();
  match *look_ahead {
    '(' => {
      if next_char() != ')' { panic!(expected("end of function parameters delimiter: )".to_string())); }
      emit_ln("CALL ".to_string() + name);
      *look_ahead = next_char();
    },
    _ => emit_ln("MOV RAX, ".to_string() + name)
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
