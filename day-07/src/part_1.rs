use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

pub fn main() {
    let input_vec = get_vector();
    let perm_vecs = get_perms();

    let mut max_signal = 0;
    for perm in perm_vecs {
        let mut output = run_program(
            input_vec.clone(),
            vec![Some(*perm.get(0).unwrap()), Some(0)],
        );
        for i in 1..5 {
            output = run_program(
                input_vec.clone(),
                vec![Some(*perm.get(i).unwrap()), Some(output)],
            );
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

fn run_program(vec: Vec<isize>, input: Vec<Option<isize>>) -> isize {
    let mut v = vec;
    let mut input_pointer = 0;
    //println!("{:?}", v);

    let mut index = 0;
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
                break output;
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
            99 => panic!("Program terminated by opcode 99"),
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

fn get_perms() -> Vec<Vec<isize>> {
    let mut perm = Vec::new();
    let mut a = 0;
    loop {
        if a > 4 {
            break;
        }
        let mut b = 0;
        loop {
            if b > 4 {
                break;
            }
            let mut c = 0;
            loop {
                if c > 4 {
                    break;
                }
                let mut d = 0;
                loop {
                    if d > 4 {
                        break;
                    }
                    let mut e = 0;
                    loop {
                        if e > 4 {
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
