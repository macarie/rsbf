use std::io::{BufRead, Write};

#[derive(Debug, PartialEq, Eq)]
pub enum BFInstruction {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    PutChar,
    ReadChar,
    JumpIfZero,
    JumpIfNotZero,
}

#[derive(Debug, PartialEq, Eq)]
pub enum VMInstruction {
    Increment(i8),
    Move(isize),
    Print,
    Read,
    Loop(Vec<VMInstruction>),
}

pub fn lexer(source_code: &String) -> Vec<BFInstruction> {
    let mut bf_instructions: Vec<BFInstruction> = Vec::new();

    for character in source_code.chars() {
        if let Some(bf_instruction) = match character {
            '+' => Some(BFInstruction::Increment),
            '-' => Some(BFInstruction::Decrement),
            '>' => Some(BFInstruction::MoveRight),
            '<' => Some(BFInstruction::MoveLeft),
            '.' => Some(BFInstruction::PutChar),
            ',' => Some(BFInstruction::ReadChar),
            '[' => Some(BFInstruction::JumpIfZero),
            ']' => Some(BFInstruction::JumpIfNotZero),
            _ => None,
        } {
            bf_instructions.push(bf_instruction)
        }
    }

    return bf_instructions;
}

fn infer_increment_direction(bf_instruction: &BFInstruction) -> i8 {
    return match bf_instruction {
        BFInstruction::Increment => 1,
        BFInstruction::Decrement => -1,
        _ => 0,
    };
}

fn infer_move_direction(bf_instruction: &BFInstruction) -> isize {
    return match bf_instruction {
        BFInstruction::MoveRight => 1,
        BFInstruction::MoveLeft => -1,
        _ => 0,
    };
}

fn parse_internal(
    bf_instructions: &[BFInstruction],
    vm_instructions: &mut Vec<VMInstruction>,
) -> usize {
    let mut index = 0;

    while index < bf_instructions.len() {
        let bf_instruction = &bf_instructions[index];
        let last_vm_instruction = vm_instructions.last_mut();

        match bf_instruction {
            BFInstruction::Increment | BFInstruction::Decrement => {
                if let Some(VMInstruction::Increment(ref mut last_increment)) = last_vm_instruction
                {
                    *last_increment += infer_increment_direction(bf_instruction);
                } else {
                    vm_instructions.push(VMInstruction::Increment(infer_increment_direction(
                        bf_instruction,
                    )));
                }
            }
            BFInstruction::MoveRight | BFInstruction::MoveLeft => {
                if let Some(VMInstruction::Move(ref mut last_move)) = last_vm_instruction {
                    *last_move += infer_move_direction(bf_instruction);
                } else {
                    vm_instructions.push(VMInstruction::Move(infer_move_direction(bf_instruction)))
                }
            }
            BFInstruction::PutChar => {
                vm_instructions.push(VMInstruction::Print);
            }
            BFInstruction::ReadChar => {
                vm_instructions.push(VMInstruction::Read);
            }
            BFInstruction::JumpIfZero => {
                let mut instructions_in_loop: Vec<VMInstruction> = Vec::new();

                let bf_instructions_consumed =
                    parse_internal(&bf_instructions[(index + 1)..], &mut instructions_in_loop);

                vm_instructions.push(VMInstruction::Loop(instructions_in_loop));

                index += bf_instructions_consumed;
            }
            BFInstruction::JumpIfNotZero => return index + 1,
        }

        index += 1
    }

    return index;
}

pub fn parse(bf_instructions: &[BFInstruction]) -> Vec<VMInstruction> {
    let mut vm_instructions: Vec<VMInstruction> = Vec::new();

    parse_internal(bf_instructions, &mut vm_instructions);

    return vm_instructions;
}

fn exec_vm_instructions(
    vm_instructions: &[VMInstruction],
    memory: &mut [u8; 30_000],
    data_pointer: &mut usize,
    reader: &mut impl BufRead,
    writer: &mut impl Write,
) {
    for vm_instruction in vm_instructions {
        match vm_instruction {
            VMInstruction::Increment(amount) => {
                memory[*data_pointer] = memory[*data_pointer].wrapping_add_signed(*amount);
            }
            VMInstruction::Move(amount) => {
                *data_pointer = data_pointer.wrapping_add_signed(*amount);
            }
            VMInstruction::Print => {
                write!(writer, "{}", memory[*data_pointer] as char).expect("cannot write");
            }
            VMInstruction::Read => {
                let mut input: [u8; 1] = [0; 1];

                reader.read_exact(&mut input).expect("cannot read");

                memory[*data_pointer] = input[0];
            }
            VMInstruction::Loop(vm_instructions) => {
                while memory[*data_pointer] != 0 {
                    exec_vm_instructions(vm_instructions, memory, data_pointer, reader, writer);
                }
            }
        }
    }
}

pub fn run(
    vm_instructions: &[VMInstruction],
    reader: &mut impl BufRead,
    writer: &mut impl Write,
) -> [u8; 30_000] {
    let mut memory: [u8; 30_000] = [0; 30_000];
    let mut data_pointer: usize = 0;

    exec_vm_instructions(
        vm_instructions,
        &mut memory,
        &mut data_pointer,
        reader,
        writer,
    );

    return memory;
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::fs;

    use crate::vm::{lexer, BFInstruction, VMInstruction};

    use super::{parse, run};

    #[test]
    fn lexer_converts_source_to_bf_instructions() {
        let source_code = fs::read_to_string("programs/hello-world.bf").unwrap();
        let bf_instructions = lexer(&source_code);

        assert_eq!(
            vec![
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::JumpIfZero,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::MoveLeft,
                BFInstruction::MoveLeft,
                BFInstruction::MoveLeft,
                BFInstruction::MoveLeft,
                BFInstruction::Decrement,
                BFInstruction::JumpIfNotZero,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::PutChar,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::MoveLeft,
                BFInstruction::MoveLeft,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::MoveRight,
                BFInstruction::PutChar,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::PutChar,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::Decrement,
                BFInstruction::PutChar,
                BFInstruction::MoveRight,
                BFInstruction::Increment,
                BFInstruction::PutChar,
                BFInstruction::MoveRight,
                BFInstruction::PutChar,
            ],
            bf_instructions,
        );
    }

    #[test]
    fn parse_converts_bf_instructions_to_vm_instructions() {
        let source_code = fs::read_to_string("programs/hello-world.bf").unwrap();
        let bf_instructions = lexer(&source_code);
        let vm_instructions = parse(&bf_instructions);

        assert_eq!(
            vec![
                VMInstruction::Increment(10),
                VMInstruction::Loop(vec![
                    VMInstruction::Move(1),
                    VMInstruction::Increment(7),
                    VMInstruction::Move(1),
                    VMInstruction::Increment(10),
                    VMInstruction::Move(1),
                    VMInstruction::Increment(3),
                    VMInstruction::Move(1),
                    VMInstruction::Increment(1),
                    VMInstruction::Move(-4),
                    VMInstruction::Increment(-1),
                ]),
                VMInstruction::Move(1),
                VMInstruction::Increment(2),
                VMInstruction::Print,
                VMInstruction::Move(1),
                VMInstruction::Increment(1),
                VMInstruction::Print,
                VMInstruction::Increment(7),
                VMInstruction::Print,
                VMInstruction::Print,
                VMInstruction::Increment(3),
                VMInstruction::Print,
                VMInstruction::Move(1),
                VMInstruction::Increment(2),
                VMInstruction::Print,
                VMInstruction::Move(-2),
                VMInstruction::Increment(15),
                VMInstruction::Print,
                VMInstruction::Move(1),
                VMInstruction::Print,
                VMInstruction::Increment(3),
                VMInstruction::Print,
                VMInstruction::Increment(-6),
                VMInstruction::Print,
                VMInstruction::Increment(-8),
                VMInstruction::Print,
                VMInstruction::Move(1),
                VMInstruction::Increment(1),
                VMInstruction::Print,
                VMInstruction::Move(1),
                VMInstruction::Print,
            ],
            vm_instructions
        );
    }

    #[test]
    fn run_successfully_executes_hello_world() {
        let mut input = "fake input".as_bytes();
        let mut output = Vec::new();

        let source_code = fs::read_to_string("programs/hello-world.bf").unwrap();
        let bf_instructions = lexer(&source_code);
        let vm_instructions = parse(&bf_instructions);

        run(&vm_instructions, &mut input, &mut output);

        assert_eq!(
            "Hello World!\n",
            String::from_utf8(output).expect("cannot convert output")
        )
    }
}
