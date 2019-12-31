use std::error::Error;
use std::fs;

pub fn main() {
    let mut moons_vec = get_moon_positions();

    for _ in 0..1000 {
        moons_vec = time_step(moons_vec);
    }
    println!("{}", get_total_energy(moons_vec));
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Moon {
    pos: [isize; 3],
    vel: [isize; 3],
}

impl Moon {
    fn new(coords: Vec<isize>) -> Moon {
        Moon {
            pos: [coords[0], coords[1], coords[2]],
            vel: [0; 3],
        }
    }

    fn apply_velocity(&mut self) {
        for coord_num in 0..3 {
            self.pos[coord_num] += self.vel[coord_num];
        }
    }

    fn calcuate_energy(&self) -> isize {
        let mut pot_energy = 0;
        let mut kin_energy = 0;
        for coord_num in 0..3 {
            pot_energy += self.pos[coord_num].abs();
            kin_energy += self.vel[coord_num].abs();
        }
        pot_energy * kin_energy
    }
}

fn get_moon_positions() -> Vec<Moon> {
    let input = read_file("./input.txt".to_string()).unwrap();
    let mut moon_vec: Vec<Moon> = Vec::new();

    for line in input.lines() {
        let iter_coord_str = line.split(",");
        let iter_numeric_str =
            iter_coord_str.map(|s| s.chars().filter(|x| x.is_numeric() || *x == '-').collect());
        let vec_position = iter_numeric_str
            .map(|x: String| x.parse().unwrap())
            .collect();
        let moon = Moon::new(vec_position);

        moon_vec.push(moon)
    }

    moon_vec
}

fn time_step(moons: Vec<Moon>) -> Vec<Moon> {
    let mut moons = apply_gravity(moons);
    for moon in &mut moons {
        moon.apply_velocity();
    }
    moons
}

fn apply_gravity(mut moons: Vec<Moon>) -> Vec<Moon> {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let (a, b) = moons.split_at_mut(j);
            let moon_i = &mut a[i];
            let moon_j = &mut b[0];
            for coord_num in 0..3 {
                if moon_i.pos[coord_num] > moon_j.pos[coord_num] {
                    moon_i.vel[coord_num] -= 1;
                    moon_j.vel[coord_num] += 1;
                }
                if moon_i.pos[coord_num] < moon_j.pos[coord_num] {
                    moon_i.vel[coord_num] += 1;
                    moon_j.vel[coord_num] -= 1;
                }
            }
        }
    }
    moons
}

fn get_total_energy(moons: Vec<Moon>) -> isize {
    moons.into_iter().map(|m| m.calcuate_energy()).sum()
}
