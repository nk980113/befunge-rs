use crate::program::Program;

pub fn parse_input(input: String) -> Program {
    let mut lines: Vec<_> = input.lines().collect();
    if lines.len() > 25 {
        panic!("Error: Program too tall");
    }
    if lines.len() < 25 {
        lines.reserve(25 - lines.len());
        for _ in lines.len()..25 {
            lines.push("");
        }
    }
    let field: Vec<[u8; 80]> = lines.into_iter().map(|l| {
        if l.len() > 80 {
            panic!("Error: Program too fat");
        }
        let len = l.len();
        let mut cs: Vec<u8> = Vec::new();
        for c in l.chars() {
            if c.is_ascii() {
                cs.push(c as u8);
            } else {
                panic!("Error: Non ASCII character");
            }
        }
        for _ in 0..(80 - len) {
            cs.push(b' ');
        }

        cs.try_into().unwrap()
    }).collect();

    Program::from_field(field.try_into().unwrap())
}
