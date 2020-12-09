use std::collections::HashSet;
use std::error::Error;
use std::fs;

use gamejoy::executor::GameJoy;
use gamejoy::executor::Machine;
use gamejoy::parser;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);

    let program = parser::parse(&input_as_str).unwrap();

    part1(&program);
    part2(&program);
    Ok(())
}

fn run_until_loop(machine: &mut GameJoy) -> Result<i32, ()> {
    let mut hit_lines: HashSet<usize> = HashSet::new();

    loop {
        if !hit_lines.insert(machine.instruction_pointer) {
            return Err(());
        }

        if let Err(code) = machine.next() {
            return Ok(code);
        }
    }
}

fn part1(program: &Vec<parser::OpCode>) {
    let mut machine = GameJoy::new(program.to_vec());
    match run_until_loop(&mut machine) {
        Ok(return_code) => println!("Part1: Function returned with code {}", return_code),
        Err(_) => println!(
            "Part1: broke at line {}. Acc: {}",
            machine.instruction_pointer, machine.accumulator
        ),
    }
}

fn part2(program: &Vec<parser::OpCode>) {
    for (index, op) in program.iter().enumerate().filter(|(_, op)| match op {
        parser::OpCode::Jmp(_) => true,
        parser::OpCode::Nop(_) => true,
        _ => false,
    }) {
        let mut prog_copy = program.clone();
        match op {
            parser::OpCode::Jmp(jmp) => {
                *prog_copy.get_mut(index).unwrap() = parser::OpCode::Nop(*jmp);
            }
            parser::OpCode::Nop(nop) => {
                *prog_copy.get_mut(index).unwrap() = parser::OpCode::Jmp(*nop);
            }
            _ => {}
        }

        let mut machine = GameJoy::new(prog_copy);

        if let Ok(return_code) = run_until_loop(&mut machine) {
            if return_code == 0 {
                println!("Part2: successfully returned! Acc: {}", machine.accumulator);
                break;
            } else {
                println!("broke due to {}", return_code);
            }
        }
    }
}
