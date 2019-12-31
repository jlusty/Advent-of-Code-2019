use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

pub fn main() {
    let input_vec = get_vector();
    run_program(Program::new(input_vec), vec![None]);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector() -> Vec<isize> {
    let input = read_file("./test.txt".to_string()).unwrap();
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn run_program(mut program: Program, input: Vec<Option<isize>>) -> (Option<isize>, Program) {
    //println!("{:?}", program.vec);

    let mut index = program.start_pointer;
    let mut input_pointer = 0;
    loop {
        let instruction = get_instruction_struct(*program.vec.get(&index).unwrap());
        match instruction.opcode {
            1 => {
                let in1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let in2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );

                let pos = *program.vec.get(&(index + 3)).unwrap();
                match *instruction.params.get(2).unwrap_or(&0) {
                    0 => {
                        program.vec.insert(pos as usize, in1 + in2);
                    }
                    2 => {
                        program
                            .vec
                            .insert((program.rel_base + pos) as usize, in1 + in2);
                    }
                    _ => panic!("Unexpected mode"),
                }

                index += 4;
            }
            2 => {
                let in1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let in2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );

                let pos = *program.vec.get(&(index + 3)).unwrap();
                match *instruction.params.get(2).unwrap_or(&0) {
                    0 => {
                        program.vec.insert(pos as usize, in1 * in2);
                    }
                    2 => {
                        program
                            .vec
                            .insert((program.rel_base + pos) as usize, in1 * in2);
                    }
                    _ => panic!("Unexpected mode"),
                }

                index += 4;
            }
            3 => {
                let input_val = input
                    .get(input_pointer)
                    .unwrap_or_else(|| &None)
                    .unwrap_or_else(|| take_input());
                input_pointer += 1;
                let pos = *program.vec.get(&(index + 1)).unwrap();
                match *instruction.params.get(0).unwrap_or(&0) {
                    0 => {
                        program.vec.insert(pos as usize, input_val);
                    }
                    2 => {
                        program
                            .vec
                            .insert((program.rel_base + pos) as usize, input_val);
                    }
                    _ => panic!("Unexpected mode"),
                }

                index += 2;
            }
            4 => {
                let output = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                // break (
                //     Some(output),
                //     Program {
                //         vec: program.vec,
                //         start_pointer: index + 2,
                //         rel_base: program.rel_base,
                //     },
                // );
                println!("{}", output);
                index += 2;
            }
            5 => {
                let param1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let param2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );
                if param1 != 0 {
                    index = param2 as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                let param1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let param2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );
                if param1 == 0 {
                    index = param2 as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let param1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let param2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );
                let param3 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 3)).unwrap(),
                    *instruction.params.get(2).unwrap_or(&0),
                );
                if param1 < param2 {
                    program.vec.insert(param3 as usize, 1);
                } else {
                    program.vec.insert(param3 as usize, 0);
                }
                index += 4;
            }
            8 => {
                let param1 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                let param2 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 2)).unwrap(),
                    *instruction.params.get(1).unwrap_or(&0),
                );
                let param3 = get_param_val(
                    &program,
                    *program.vec.get(&(index + 3)).unwrap(),
                    *instruction.params.get(2).unwrap_or(&0),
                );
                if param1 == param2 {
                    program.vec.insert(param3 as usize, 1);
                } else {
                    program.vec.insert(param3 as usize, 0);
                }
                index += 4;
            }
            9 => {
                let param = get_param_val(
                    &program,
                    *program.vec.get(&(index + 1)).unwrap(),
                    *instruction.params.get(0).unwrap_or(&0),
                );
                program.rel_base += param;
                index += 2;
            }
            99 => {
                break (
                    None,
                    Program {
                        vec: program.vec,
                        start_pointer: index,
                        rel_base: program.rel_base,
                    },
                )
            }
            _ => panic!("Invalid opcode"),
        }
        //println!("{:?}", v);
    }
}

fn take_input() -> isize {
    print!("Enter some input: ");
    io::stdout().flush().unwrap();
    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    input_str.trim().parse().unwrap()
}

fn get_param_val(program: &Program, param: isize, mode: isize) -> isize {
    println!(
        "{:?},{},{}",
        program.vec.get(&(param as usize)).unwrap_or(&56758768),
        param,
        mode
    );
    match mode {
        0 => *program.vec.get(&(param as usize)).unwrap(),
        1 => param,
        2 => *program
            .vec
            .get(&((program.rel_base + param) as usize))
            .unwrap(),
        _ => panic!("Invalid mode"),
    }
}

fn get_instruction_struct(instruction: isize) -> Instruction {
    //println!("{}", instruction);
    let instruction_str = instruction.to_string();
    let ins_len = instruction_str.len();
    let opcode_str;
    let mut params_str = Vec::new();
    if ins_len > 1 {
        opcode_str = &instruction_str[ins_len - 2..ins_len];
        for i in (0..ins_len - 2).rev() {
            params_str.push(&instruction_str[i..i + 1]);
        }
    } else {
        opcode_str = &instruction_str[ins_len - 1..ins_len];
    }
    Instruction {
        opcode: opcode_str.parse().unwrap(),
        params: params_str.into_iter().map(|x| x.parse().unwrap()).collect(),
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: isize,
    params: Vec<isize>,
}

#[derive(Debug)]
struct Program {
    vec: HashMap<usize, isize>,
    start_pointer: usize,
    rel_base: isize,
}

impl Program {
    fn new(vec: Vec<isize>) -> Program {
        let mut prealloc = HashMap::new();
        let mut index = 0;
        vec.into_iter().enumerate().for_each(|(i, val)| {
            prealloc.insert(i, val);
            index = i;
        });
        for i in index + 2..index + 1000 {
            prealloc.insert(i, 0);
        }

        Program {
            vec: prealloc,
            start_pointer: 0,
            rel_base: 0,
        }
    }
}
