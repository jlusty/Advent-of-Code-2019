use std::error::Error;
use std::fs;
use std::isize;

pub fn main() {
    let input = read_file("./input.txt".to_string()).unwrap();
    let wire1 = get_vector(input.lines().next().unwrap());
    let wire2 = get_vector(input.lines().skip(1).next().unwrap());

    let mut pos1 = get_pos(wire1);
    let mut pos2 = get_pos(wire2);

    pos1.sort_by(|a, b| a.1.cmp(&b.1));
    pos2.sort_by(|a, b| a.1.cmp(&b.1));
    pos1.sort_by(|a, b| a.0.cmp(&b.0));
    pos2.sort_by(|a, b| a.0.cmp(&b.0));

    let mut distance_vec: Vec<isize> = Vec::new();
    let mut i_idx = 0;
    let mut j_idx = 0;
    let mut i;
    let mut j;
    while i_idx < pos1.len() && j_idx < pos2.len() {
        i = pos1[i_idx];
        j = pos2[j_idx];
        if i.0 < j.0 {
            i_idx += 1
        } else if j.0 < i.0 {
            j_idx += 1
        } else {
            if i.1 < j.1 {
                i_idx += 1
            } else if j.1 < i.1 {
                j_idx += 1
            } else {
                //println!("{:?}", i);
                //println!("{:?}", i.2 + j.2);
                i_idx += 1;
                j_idx += 1;
                distance_vec.push(i.2 + j.2)
            }
        }
    }
    distance_vec.sort();
    println!("{:?}", distance_vec);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_vector(line: &str) -> Vec<(char, isize)> {
    let tup_vec: Vec<(char, String)> = line
        .split(",")
        .map(|x| (x.chars().next().unwrap(), x.chars().skip(1).collect()))
        .collect();
    tup_vec
        .into_iter()
        .map(|(p, q)| (p, q.parse().unwrap()))
        .collect()
}

fn get_pos(wire: Vec<(char, isize)>) -> Vec<(isize, isize, isize)> {
    let mut pos_vec: Vec<(isize, isize, isize)> = Vec::new();
    pos_vec.push((0, 0, 0));
    let mut current_pos = pos_vec.last().unwrap().clone();
    let mut steps_taken = 1;

    for i in wire.into_iter() {
        match i.0 {
            'U' => {
                for y in 1..i.1 + 1 {
                    pos_vec.push((current_pos.0, current_pos.1 + y, steps_taken));
                    steps_taken += 1;
                }
                current_pos = pos_vec.last().unwrap().clone();
            }
            'D' => {
                for y in 1..i.1 + 1 {
                    pos_vec.push((current_pos.0, current_pos.1 - y, steps_taken));
                    steps_taken += 1;
                }
                current_pos = pos_vec.last().unwrap().clone();
            }

            'L' => {
                for x in 1..i.1 + 1 {
                    pos_vec.push((current_pos.0 - x, current_pos.1, steps_taken));
                    steps_taken += 1;
                }
                current_pos = pos_vec.last().unwrap().clone();
            }
            'R' => {
                for x in 1..i.1 + 1 {
                    pos_vec.push((current_pos.0 + x, current_pos.1, steps_taken));
                    steps_taken += 1;
                }
                current_pos = pos_vec.last().unwrap().clone();
            }
            _ => panic!("Unrecognised char"),
        }
    }
    pos_vec
}
