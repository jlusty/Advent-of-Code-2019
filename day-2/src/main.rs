use std::error::Error;
use std::fs;

fn main() {
    println!("{}", run_program(get_vector())[0]);

    let vec = get_vector();
    let mut noun = 0;
    let mut verb = 0;
    loop {
        loop {
            let mut new_vec = vec.clone();
            new_vec[1] = noun;
            new_vec[2] = verb;

            if run_program(new_vec)[0] == 19690720 {
                println!("{}, {}, {}", noun, verb, 100 * noun + verb);
                panic!("Done!")
            };
            if verb < 100 {
                verb += 1;
            } else {
                verb = 0;
                break;
            }
        }
        noun += 1;
    }
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector() -> Vec<usize> {
    let input = read_file("./input.txt".to_string()).unwrap();
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn run_program(vec: Vec<usize>) -> Vec<usize> {
    let mut v = vec;
    //println!("{:?}", v);

    let mut index = 0;
    loop {
        match v[index] {
            1 => {
                let in1 = v[index + 1];
                let in2 = v[index + 2];
                let pos = v[index + 3];
                v[pos] = v[in1] + v[in2];
            }
            2 => {
                let in1 = v[index + 1];
                let in2 = v[index + 2];
                let pos = v[index + 3];
                v[pos] = v[in1] * v[in2];
            }
            99 => break v,
            _ => panic!("Invalid opcode"),
        }
        //println!("{:?}", v);
        index += 4;
    }
}
