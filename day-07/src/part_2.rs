use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

pub fn main() {
    let input_vec = get_vector();
    let perm_vecs = get_perms();

    let mut max_signal = 0;
    for perm in perm_vecs {
        let mut amp_state: Vec<Program> = Vec::new();
        for _ in 0..5 {
            amp_state.push(new_program(input_vec.clone()))
        }
        let mut output = 0;

        for i in 0..5 {
            let program_return = run_program(
                &amp_state[i],
                vec![Some(*perm.get(i).unwrap()), Some(output)],
            );
            output = program_return.0.unwrap();
            amp_state[i] = program_return.1;
        }
        let mut i = 0;
        loop {
            let program_return = run_program(&amp_state[i], vec![Some(output)]);
            output = match program_return.0 {
                Some(output) => output,
                None => break,
            };
            amp_state[i] = program_return.1;
            i = if i >= 4 { 0 } else { i + 1 };
        }

        if output > max_signal {
            max_signal = output;
        }
    }
    println!("{}", max_signal);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector() -> Vec<isize> {
    let input = read_file("./input.txt".to_string()).unwrap();
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn run_program(program: &Program, input: Vec<Option<isize>>) -> (Option<isize>, Program) {
    let mut v = program.vec.clone();
    let mut input_pointer = 0;
    //println!("{:?}", v);

    let mut index = program.start_pointer;
    loop {
        let instruction = get_instruction_struct(v[index]);
        match instruction.opcode {
            1 => {
                let in1 = get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let in2 = get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                let pos = v[index + 3];
                v[pos as usize] = in1 + in2;
                index += 4;
            }
            2 => {
                let in1 = get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let in2 = get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                let pos = v[index + 3];
                v[pos as usize] = in1 * in2;
                index += 4;
            }
            3 => {
                let input_val = input
                    .get(input_pointer)
                    .unwrap_or_else(|| &None)
                    .unwrap_or_else(|| take_input());
                input_pointer += 1;
                let pos = v[index + 1];
                v[pos as usize] = input_val;
                index += 2;
            }
            4 => {
                let output =
                    get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                break (
                    Some(output),
                    Program {
                        vec: v,
                        start_pointer: index + 2,
                    },
                );
                //println!("{}", output);
                //index += 2;
            }
            5 => {
                let param1 =
                    get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let param2 =
                    get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                if param1 != 0 {
                    index = param2 as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                let param1 =
                    get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let param2 =
                    get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                if param1 == 0 {
                    index = param2 as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let param1 =
                    get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let param2 =
                    get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                let param3 = v[index + 3];
                if param1 < param2 {
                    v[param3 as usize] = 1;
                } else {
                    v[param3 as usize] = 0;
                }
                index += 4;
            }
            8 => {
                let param1 =
                    get_param_val(&v, v[index + 1], *instruction.params.get(0).unwrap_or(&0));
                let param2 =
                    get_param_val(&v, v[index + 2], *instruction.params.get(1).unwrap_or(&0));
                let param3 = v[index + 3];
                if param1 == param2 {
                    v[param3 as usize] = 1;
                } else {
                    v[param3 as usize] = 0;
                }
                index += 4;
            }
            99 => {
                break (
                    None,
                    Program {
                        vec: v,
                        start_pointer: index,
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

fn get_param_val(vec: &Vec<isize>, param: isize, mode: isize) -> isize {
    match mode {
        0 => vec[param as usize],
        1 => param,
        _ => panic!("Invalid mode"),
    }
}

fn get_instruction_struct(instruction: isize) -> Instruction {
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
    vec: Vec<isize>,
    start_pointer: usize,
}

fn new_program(vec: Vec<isize>) -> Program {
    Program {
        vec,
        start_pointer: 0,
    }
}

fn get_perms() -> Vec<Vec<isize>> {
    let mut perm = Vec::new();
    let mut a = 5;
    loop {
        if a > 9 {
            break;
        }
        let mut b = 5;
        loop {
            if b > 9 {
                break;
            }
            let mut c = 5;
            loop {
                if c > 9 {
                    break;
                }
                let mut d = 5;
                loop {
                    if d > 9 {
                        break;
                    }
                    let mut e = 5;
                    loop {
                        if e > 9 {
                            break;
                        }
                        if !(a == b
                            || a == c
                            || a == d
                            || a == e
                            || b == c
                            || b == d
                            || b == e
                            || c == d
                            || c == e
                            || d == e)
                        {
                            perm.push(vec![a, b, c, d, e]);
                        }
                        e += 1;
                    }
                    d += 1;
                }
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    perm
}
