use std::io::stdin;

fn main() {
  let mut look_ahead = ' ';

  emit_ln("section .text");
  emit_ln("global _start");
  emit_ln("_start:");
  emit_ln(";; start of code gen");

  assignment(&mut look_ahead);
  if look_ahead != '\n' {panic!(expected("Newline as the end of the input"));}

  emit_ln("\n\t;; end of code gen\n\tMOV RAX, 60");
  emit_ln("XOR RDI, RDI");
  emit_ln("SYSCALL");
}

fn expression(look_ahead: &mut char) {
  term(look_ahead);
  while vec!('+', '-').contains(look_ahead) {
    emit_ln("PUSH RAX");
    match *look_ahead {
      '+' => add(look_ahead),
      '-' => sub(look_ahead),
      _   => panic!(expected("addop"))
    }
  }
}

fn assignment(look_ahead: &mut char) {
  *look_ahead = next_char();
  let name = get_name(look_ahead);

  if *look_ahead != '=' {panic!(expected("="));}
  expression(look_ahead);

  let instr = "MOV ".to_string() + name + ", RAX".to_string();
  emit_ln(instr.as_slice());
}

fn term(look_ahead: &mut char) {
  factor(look_ahead);
  while vec!('*', '/').contains(look_ahead) {
    emit_ln("PUSH RAX");
    match *look_ahead {
      '*' => mul(look_ahead),
      '/' => div(look_ahead),
      _   => panic!(expected("mulop"))
    }
  }
}

fn add(look_ahead: &mut char) {
  term(look_ahead);
  emit_ln("POP RCX");
  emit_ln("ADD RAX, RCX");
}

fn sub(look_ahead: &mut char) {
  term(look_ahead);
  emit_ln("POP RCX");
  emit_ln("SUB RAX, RCX");
  emit_ln("NEG RAX");
}

fn mul(look_ahead: &mut char) {
  factor(look_ahead);
  emit_ln("POP RCX");
  emit_ln("IMUL RCX");
}

fn div(look_ahead: &mut char) {
  factor(look_ahead);
  emit_ln("MOV RCX, RAX");
  emit_ln("POP RAX");
  emit_ln("XOR RDX, RDX");
  emit_ln("IDIV RCX");
}

fn factor(look_ahead: &mut char) {
  *look_ahead = next_char();

  match *look_ahead {
    '(' => {
      expression(look_ahead);

      if *look_ahead != ')' {
        panic!(expected("delimiter: )"));
      }

      *look_ahead = next_char();
    },
    '+'|'-'                => emit_ln("XOR RAX, RAX"),
    x if x.is_alphabetic() => ident(look_ahead),
    _                      => {
      let instr = "MOV RAX, ".to_string() + get_number(look_ahead).to_string();
      emit_ln(instr.as_slice());
    }
  }
}

fn next_char() -> char {
  let mut next = stdin().read_char().unwrap();
  while next == ' ' {
    next = stdin().read_char().unwrap();
  }
  next
}

fn get_number(look_ahead: &mut char) -> uint {
  let mut nb = 0;
  if !look_ahead.is_digit(10) {panic!(expected("integer"));}

  while look_ahead.is_digit(10) {
    nb = 10*nb + look_ahead.to_digit(10).unwrap();
    *look_ahead = next_char();
  }
  nb
}

fn get_name(look_ahead: &mut char) -> String {
  let mut name = look_ahead.to_string();
  *look_ahead = next_char();
  while (*look_ahead).is_alphanumeric() {
    name = name + look_ahead.to_string();
    *look_ahead = next_char();
  }
  name
}

fn ident(look_ahead: &mut char) {
  let name = get_name(look_ahead);
  match *look_ahead {
    '(' => {
      if next_char() != ')' { panic!(expected("end of function parameters delimiter: )")); }
      let instr = "CALL ".to_string() + name;
      emit_ln(instr.as_slice());
      *look_ahead = next_char();
    },
    _ => {
      let instr = "MOV RAX, ".to_string() + name;
      emit_ln(instr.as_slice());
    }
  }
}

fn error(s: String) -> String {
  format!("Error: {}.", s)
}

fn expected(s: &str) -> String {
  error(s.to_string() + " expected")
}

fn emit(s: String) {
  print!("\t{}", s);
}

fn emit_ln(s: &str) {
  emit(s.to_string() + "\n".to_string());
}
