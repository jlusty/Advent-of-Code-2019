use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

pub fn main() {
    let input_vec = get_vector();
    run_program(input_vec);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector() -> Vec<isize> {
    let input = read_file("./input.txt".to_string()).unwrap();
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn run_program(vec: Vec<isize>) -> Vec<isize> {
    let mut v = vec;
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
                let input = take_input();
                let pos = v[index + 1];
                v[pos as usize] = input;
                index += 2;
            }
            4 => {
                let pos = v[index + 1];
                println!("{}", v[pos as usize]);
                index += 2;
            }
            99 => break v,
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
