use crate::program::{Program, Direction};
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Stdin, stdin, Read, BufRead};

pub fn execute(program: Program) {
    let mut program = program;
    loop {
        match program.current_point() {
            // End
            b'@' => break,
            // Space
            b' ' => (),
            // Direction
            b'^' => program.set_dir(Direction::Up),
            b'v' => program.set_dir(Direction::Down),
            b'<' => program.set_dir(Direction::Left),
            b'>' => program.set_dir(Direction::Right),
            b'?' => program.set_dir(match ((SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() % 4) + (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() % 4)) % 4 {
                0 => Direction::Up,
                1 => Direction::Right,
                2 => Direction::Down,
                3 => Direction::Left,
                _ => unreachable!(),
            }),
            // Output
            b'.' => print!("{}", program.pop()),
            b',' => print!("{}", program.pop() as u8 as char),
            // Input
            b'&' => {
                let mut s = String::new();
                Stdin::lock(&stdin()).read_line(&mut s).unwrap();
                let _ = s.split_off(s.len() - 2);
                match s.parse() {
                    Ok(v) => program.push(v),
                    Err(_) => panic!("Expected number to be entered"),
                }
            },
            b'~' => {
                program.push(stdin().bytes().next().and_then(Result::ok).unwrap() as i64);
            },
            b'"' => {
                loop {
                    program.step();
                    match program.current_point() {
                        b'"' => break,
                        c => program.push(c as i64),
                    }
                }
            },
            n @ b'0'..=b'9' => program.push((n - b'0') as i64),
            // Arithmetics
            b'+' => {
                let res = program.pop() + program.pop();
                program.push(res);
            },
            b'-' => {
                let v2 = program.pop();
                let v1 = program.pop();
                program.push(v1 - v2);
            },
            b'*' => {
                let res = program.pop() * program.pop();
                program.push(res);
            },
            b'%' => {
                let v2 = program.pop();
                let v1 = program.pop();
                program.push(v1 % v2);
            },
            b'!' => {
                let res = (program.pop() == 0) as i64;
                program.push(res);
            },
            b'`' => {
                let res = (program.pop() < program.pop()) as i64;
                program.push(res);
            },
            b'/' => {
                let v2 = program.pop();
                let v1 = program.pop();
                if v2 == 0 {
                    let mut s = String::new();
                    print!("Divide by zero exception; Enter the new value: ");
                    stdin().read_line(&mut s).unwrap();
                    match s.parse() {
                        Ok(v) => program.push(v),
                        Err(_) => panic!("Expected number to be entered"),
                    }
                } else {
                    program.push(v1 / v2);
                }
            },
            // Stack arithmetics
            b':' => {
                let v = program.pop();
                program.push(v);
                program.push(v);
            },
            b'\\' => {
                let v2 = program.pop();
                let v1 = program.pop();
                program.push(v2);
                program.push(v1);
            },
            b'$' => {
                program.pop();
            },
            // Conditionals
            b'_' => if program.pop() == 0 {
                program.set_dir(Direction::Right);
            } else {
                program.set_dir(Direction::Left);
            },
            b'|' => if program.pop() == 0 {
                program.set_dir(Direction::Down);
            } else {
                program.set_dir(Direction::Up);
            },
            // Field manipulating
            b'#' => program.step(),
            b'g' => {
                let y = program.pop();
                let x = program.pop();
                let v = program.point(x.try_into().unwrap(), y.try_into().unwrap());
                program.push(v);
            },
            b'p' => {
                let y = program.pop();
                let x = program.pop();
                let v = program.pop();
                program.set_point(x.try_into().unwrap(), y.try_into().unwrap(), v.try_into().unwrap());
            }

            _ => panic!("Unexpected character")
        }

        program.step();
    }
}
