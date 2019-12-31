use rand::Rng;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::Write;

const GRID_SIZE: usize = 50;
pub fn main() {
    let mut prog = Program::new(get_vector());
    let mut panels = Panels::new();
    let mut rng = rand::thread_rng();

    let mut i = 0;
    loop {
        let (status_, prog_) = run_program(prog, Input::new(vec![panels.current_dir_val()]));
        prog = prog_;
        let status = match status_ {
            Some(num) => num,
            None => break,
        };

        //panels.wall(status);
        //panels.turn(direction_val);
        //println!("{}", status);
        //println!("{:?}", panels.dir);
        let moved = panels.attempt_move(status);
        // println!("{:?}", panels.history);
        // println!("{}", panels);
        // if i % 1000 == 0 {
        //     println!("{}", panels);
        // }

        if !moved {
            // if panels.current_visit_count() < 2 {
            //     panels.turn(1);
            // } else {
            //     if rng.gen_bool(0.5) {
            //         panels.turn(0);
            //     } else {
            //         panels.turn(1);
            //     }
            // }

            if rng.gen_bool(0.5) {
                panels.turn(0);
            } else {
                panels.turn(1);
            }
        } else {
            if rng.gen_bool(0.1) {
                panels.turn(0);
            }
        }
        i += 1;
    }

    println!("{}", panels);

    //let mut total_painted = 0;
    // for row in panels.was_painted.iter() {
    //     for x in row.iter() {
    //         if *x {
    //             total_painted += 1;
    //         }
    //     }
    // }
    //println!("{}", total_painted);
    //println!("{:?}", run_program(Program::new(input_vec), Input::new()));
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}
impl Direction {
    fn clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    fn anticlockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

struct Panels {
    grid: [[isize; GRID_SIZE]; GRID_SIZE],
    history: Vec<(usize, usize)>,
    pos: (usize, usize),
    dir: Direction,
}
impl Panels {
    fn new() -> Panels {
        let mut panels = Panels {
            grid: [[-1; GRID_SIZE]; GRID_SIZE],
            history: Vec::new(),
            pos: (GRID_SIZE / 2, GRID_SIZE / 2),
            dir: Direction::North,
        };
        panels.grid[panels.pos.1][panels.pos.0] = 1;
        panels
    }
    fn turn(&mut self, val: isize) {
        self.dir = match val {
            0 => self.dir.anticlockwise(),
            1 => self.dir.clockwise(),
            _ => panic!("Unknown direction value"),
        }
    }
    fn attempt_move(&mut self, status: isize) -> bool {
        match status {
            0 => {
                self.wall();
                false
            }
            1 => {
                match &self.dir {
                    Direction::North => self.pos.1 -= 1,
                    Direction::West => self.pos.0 -= 1,
                    Direction::South => self.pos.1 += 1,
                    Direction::East => self.pos.0 += 1,
                };
                let pos_val = &mut self.grid[self.pos.1][self.pos.0];
                if *pos_val == -1 {
                    *pos_val = 1;
                } else {
                    *pos_val += 1;
                }
                //Check history of movement
                match self.history.iter().position(|&p| p == self.pos) {
                    Some(i) => {
                        self.history.truncate(i + 1);
                    }
                    None => {
                        self.history.push(self.pos);
                    }
                }
                if self.pos == (GRID_SIZE / 2, GRID_SIZE / 2) {
                    self.history.clear();
                }

                true
            }
            2 => {
                println!("{:?}", self.history);
                println!("{}", self.history.len() + 1);
                panic!("Found oxygen system");
            }
            _ => panic!("Unrecognized status code"),
        }
    }
    fn wall(&mut self) {
        // self.was_painted[self.pos.1][self.pos.0] = true;
        let mut in_front = self.pos;
        match &self.dir {
            Direction::North => in_front.1 -= 1,
            Direction::West => in_front.0 -= 1,
            Direction::South => in_front.1 += 1,
            Direction::East => in_front.0 += 1,
        }
        self.grid[in_front.1][in_front.0] = 0;
    }
    // fn lots_of_times(&mut self) {
    //     self.grid[self.pos.1][self.pos.0] = -999;
    // }
    fn current_visit_count(&self) -> isize {
        self.grid[self.pos.1][self.pos.0]
    }
    fn current_dir_val(&self) -> isize {
        self.dir.clone() as isize
    }
}
impl fmt::Display for Panels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.pos == (x, y) {
                    match &self.dir {
                        Direction::North => write!(f, "^").unwrap(),
                        Direction::West => write!(f, "<").unwrap(),
                        Direction::South => write!(f, "v").unwrap(),
                        Direction::East => write!(f, ">").unwrap(),
                    };
                } else if (GRID_SIZE / 2, GRID_SIZE / 2) == (x, y) {
                    write!(f, "s").unwrap()
                } else if self.history.contains(&(x, y)) {
                    write!(f, "o").unwrap()
                } else {
                    match self.grid[y][x] {
                        -1 => write!(f, " ").unwrap(),
                        0 => write!(f, "#").unwrap(),
                        _ => write!(f, ".").unwrap(),
                    }
                }
            }
            writeln!(f, "").unwrap();
        }
        writeln!(f, "")
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
    fn new(values: Vec<isize>) -> Input {
        Input {
            values,
            pointer: Some(0),
        }
    }

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
