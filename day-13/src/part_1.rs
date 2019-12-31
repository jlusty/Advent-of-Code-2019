use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;

pub fn main() {
    let mut prog = Program::new(get_vector());

    let mut total_blocks = 0;
    loop {
        let (out1, prog_) = run_program(prog, Input::none());
        prog = prog_;
        let out1 = match out1 {
            Some(i) => i,
            None => break,
        };
        let (out2, prog_) = run_program(prog, Input::none());
        prog = prog_;
        let out2 = match out2 {
            Some(i) => i,
            None => break,
        };
        let (out3, prog_) = run_program(prog, Input::none());
        prog = prog_;
        let out3 = match out3 {
            Some(i) => i,
            None => break,
        };

        let output = Output::new(out1, out2, out3);
        if output.id == 2 {
            total_blocks += 1;
        }

        //println!("{}", panels);
    }

    println!("{}", total_blocks);
    //println!("{:?}", run_program(Program::new(input_vec), Input::new()));
}

struct Output {
    //x: isize,
    //y: isize,
    id: isize,
}
impl Output {
    fn new(_x: isize, _y: isize, id: isize) -> Output {
        Output {
            //x,
            //y,
            id,
        }
    }
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector() -> Vec<isize> {
    let input = read_file("./input.txt".to_string()).unwrap();
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn run_program(mut prog: Program, mut input: Input) -> (Option<isize>, Program) {
    loop {
        let ins = prog.next_instruction();
        //println!("{:?}", ins);
        //println!("{:?}", prog);
        match ins.opcode {
            1 => {
                prog.write(
                    &ins.params[2],
                    ins.params[0].get(&prog) + ins.params[1].get(&prog),
                );
            }
            2 => {
                prog.write(
                    &ins.params[2],
                    ins.params[0].get(&prog) * ins.params[1].get(&prog),
                );
            }
            3 => {
                let input_val = match input.pointer {
                    Some(_) => input.next(),
                    None => Input::take(),
                };
                prog.write(&ins.params[0], input_val);
            }
            4 => {
                break (Some(ins.params[0].get(&prog)), prog);
                //println!("{}", ins.params[0].get(&prog));
            }
            5 => {
                if ins.params[0].get(&prog) != 0 {
                    prog.jump_to(&ins.params[1]);
                }
            }
            6 => {
                if ins.params[0].get(&prog) == 0 {
                    prog.jump_to(&ins.params[1]);
                }
            }
            7 => {
                prog.write(
                    &ins.params[2],
                    if ins.params[0].get(&prog) < ins.params[1].get(&prog) {
                        1
                    } else {
                        0
                    },
                );
            }
            8 => {
                prog.write(
                    &ins.params[2],
                    if ins.params[0].get(&prog) == ins.params[1].get(&prog) {
                        1
                    } else {
                        0
                    },
                );
            }
            9 => {
                prog.modify_rel_base(&ins.params[0]);
            }
            99 => break (None, prog),
            _ => panic!("Invalid opcode"),
        }
    }
}

struct Input {
    values: Vec<isize>,
    pointer: Option<usize>,
}
impl Input {
    fn none() -> Input {
        Input {
            values: Vec::new(),
            pointer: None,
        }
    }

    fn take() -> isize {
        print!("Enter some input: ");
        io::stdout().flush().unwrap();
        let mut input_str = String::new();
        io::stdin()
            .read_line(&mut input_str)
            .expect("Failed to read line");
        input_str.trim().parse().unwrap()
    }

    fn next(&mut self) -> isize {
        let val = *self.values.get(self.pointer.unwrap()).unwrap();
        self.pointer = Some(self.pointer.unwrap() + 1);
        if self.pointer.unwrap() >= self.values.len() {
            self.pointer = None;
        }
        val
    }
}

#[derive(Debug)]
enum Param {
    Position(usize),
    Immediate(isize),
    Relative(usize),
}
impl Param {
    fn get(&self, program: &Program) -> isize {
        match self {
            Param::Position(i) => *program.vec.get(*i).unwrap(),
            Param::Immediate(i) => *i,
            Param::Relative(i) => *program
                .vec
                .get((program.rel_base as isize + *i as isize) as usize)
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    params: Vec<Param>,
}
impl Instruction {
    fn new(opcode: usize, params: Vec<Param>) -> Instruction {
        Instruction { opcode, params }
    }

    fn param_count(opcode: usize) -> usize {
        match opcode {
            1 => 3,
            2 => 3,
            3 => 1,
            4 => 1,
            5 => 2,
            6 => 2,
            7 => 3,
            8 => 3,
            9 => 1,
            99 => 0,
            _ => panic!("Unknown opcode"),
        }
    }
}

#[derive(Debug)]
struct Program {
    vec: Vec<isize>,
    index: usize,
    rel_base: usize,
    jumped: bool,
}

impl Program {
    fn new(mut vec: Vec<isize>) -> Program {
        vec.extend(vec![0; 10000]);
        Program {
            vec,
            index: 0,
            rel_base: 0,
            jumped: false,
        }
    }

    fn parse_value(&self) -> (usize, Vec<usize>) {
        let instruction_str = self.vec[self.index].to_string();
        let ins_len = instruction_str.len();
        let opcode_str;
        let mut modes_str = Vec::new();
        if ins_len > 1 {
            opcode_str = &instruction_str[ins_len - 2..ins_len];
            for i in (0..ins_len - 2).rev() {
                modes_str.push(&instruction_str[i..i + 1]);
            }
        } else {
            opcode_str = &instruction_str[ins_len - 1..ins_len];
        }

        let opcode = opcode_str.parse().unwrap();
        let mut modes: Vec<usize> = modes_str.into_iter().map(|x| x.parse().unwrap()).collect();
        while modes.len() < Instruction::param_count(opcode) {
            modes.push(0);
        }

        (opcode, modes)
    }

    fn get_instruction(&self, index: &usize) -> Instruction {
        let (opcode, modes) = self.parse_value();
        let mut params: Vec<Param> = Vec::new();
        for i in 0..Instruction::param_count(opcode) {
            let value = *self.vec.get(index + 1 + i).unwrap();
            let param = match modes.get(i).unwrap() {
                0 => Param::Position(value as usize),
                1 => Param::Immediate(value),
                2 => Param::Relative(value as usize),
                _ => panic!("Invalid mode"),
            };
            params.push(param);
        }

        Instruction::new(opcode, params)
    }

    fn next_instruction(&mut self) -> Instruction {
        // if self.jumped {
        //     self.jumped = false;
        //     self.get_instruction(&self.index)
        // } else {
        let instruction = self.get_instruction(&self.index);
        self.index += Instruction::param_count(instruction.opcode) + 1;
        instruction
        //}
    }

    fn write(&mut self, index: &Param, value: isize) {
        match index {
            Param::Position(i) => self.vec[*i] = value,
            Param::Immediate(_) => panic!("Can't write in immediate mode"),
            Param::Relative(i) => self.vec[(self.rel_base as isize + *i as isize) as usize] = value,
        };
    }

    fn jump_to(&mut self, param: &Param) {
        self.index = param.get(self) as usize;
        self.jumped = true;
    }

    fn modify_rel_base(&mut self, param: &Param) {
        self.rel_base = (self.rel_base as isize + param.get(self) as isize) as usize;
    }
}
