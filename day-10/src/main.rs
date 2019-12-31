extern crate num;

use num::integer::gcd;
use std::error::Error;
use std::fmt;
use std::fs;

fn main() {
    let asteroids = get_asteroid_map();
    let mut max_count = 0;
    let mut best_pos = Position::new(0, 0);
    let mut best_map = asteroids.clone();

    for x in 0..asteroids.width() {
        for y in 0..asteroids.height() {
            // let x = 4;
            // let y = 0;
            if asteroids.at(x, y) == '#' {
                let map = get_visible(&Position::new(x, y), asteroids.clone());
                let count = map.count();
                if count > max_count {
                    max_count = count;
                    best_pos = Position::new(x, y);
                    best_map = map;
                }
            }
        }
    }

    println!("Count: {}, {:?}", max_count - 1, best_pos); //To account for asteroid we are on
    println!("{}", best_map); //To account for asteroid we are on

    part_2(best_pos, best_map);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_asteroid_map() -> AsteroidMap {
    let input = read_file("./input.txt".to_string()).unwrap();
    let mut asteroid_map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        asteroid_map.push(line.chars().collect());
    }

    AsteroidMap::new(asteroid_map)
}

fn get_visible(station: &Position, mut asteroids: AsteroidMap) -> AsteroidMap {
    for x in 0..asteroids.width() {
        for y in 0..asteroids.height() {
            if asteroids.at(x, y) == '#' {
                block_out_path(&station, &Position::new(x, y), &mut asteroids)
            }
        }
    }
    asteroids
}

#[derive(PartialEq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

#[derive(Clone)]
struct AsteroidMap {
    map: Vec<Vec<char>>,
}

impl AsteroidMap {
    fn new(map: Vec<Vec<char>>) -> AsteroidMap {
        AsteroidMap { map }
    }

    fn width(&self) -> usize {
        self.map.get(0).unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn delete_asteroid(&mut self, x: usize, y: usize) {
        self.map[y][x] = 'x';
    }

    fn at(&self, x: usize, y: usize) -> char {
        self.map[y][x]
    }

    fn count(&self) -> usize {
        self.map
            .iter()
            .map(|row| row.iter().filter(|x| x == &&'#').count())
            .sum()
    }
}

impl fmt::Display for AsteroidMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "").unwrap();
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self.map[y][x]).unwrap();
            }
            writeln!(f, "").unwrap();
        }
        Ok(())
    }
}

fn block_out_path(station: &Position, asteroid: &Position, asteroid_map: &mut AsteroidMap) {
    if station.x == asteroid.x && station.y == asteroid.y {
        return;
    }
    let mut x_inc = asteroid.x as isize - station.x as isize;
    let mut y_inc = asteroid.y as isize - station.y as isize;

    if x_inc == 0 {
        y_inc = y_inc / y_inc.abs();
    }
    if y_inc == 0 {
        x_inc = x_inc / x_inc.abs();
    }
    while gcd(x_inc, y_inc) > 1 {
        let gcd = gcd(x_inc, y_inc);
        x_inc = x_inc / gcd;
        y_inc = y_inc / gcd;
    }

    let mut x = asteroid.x as isize + x_inc;
    let mut y = asteroid.y as isize + y_inc;
    //println!("{:?}, {}, {}", asteroid, x_inc, y_inc);
    while x >= 0
        && y >= 0
        && x < asteroid_map.width() as isize
        && y < asteroid_map.height() as isize
    {
        if asteroid_map.at(x as usize, y as usize) == '#' {
            asteroid_map.delete_asteroid(x as usize, y as usize);
        }
        //println!("{}, {}", x, y);
        x += x_inc;
        y += y_inc;
    }
}

fn part_2(station: Position, map: AsteroidMap) {
    let mut gradients: Vec<(f64, f64, f64)> = Vec::new();
    let mut points: Vec<Position> = Vec::new();

    let asteroids = map;
    for x in 0..asteroids.width() {
        for y in 0..asteroids.height() {
            if asteroids.at(x, y) == '#' {
                let asteroid = Position::new(x, y);

                if (asteroid.x as f64 - station.x as f64) != 0.0 {
                    let delta_x = asteroid.x as f64 - station.x as f64;
                    let delta_y = -(asteroid.y as f64 - station.y as f64);
                    let grad = delta_y / delta_x;
                    gradients.push((grad, delta_x, delta_y));
                    points.push(asteroid);
                }
            }
        }
    }

    //let pos_grads = gradients.iter().filter(|x| x >)

    let mut quartile: [Vec<(f64, f64, f64)>; 4] = [
        vec![(0.0, 0.0, 0.0)],
        vec![(0.0, 0.0, 0.0)],
        vec![(0.0, 0.0, 0.0)],
        vec![(0.0, 0.0, 0.0)],
    ];

    quartile[0] = gradients
        .iter()
        .filter(|(grad, delta_x, delta_y)| delta_x > &0.0 && delta_y >= &0.0)
        .map(|x| *x)
        .collect();
    quartile[1] = gradients
        .iter()
        .filter(|(grad, delta_x, delta_y)| delta_x > &0.0 && delta_y < &0.0)
        .map(|x| *x)
        .collect();
    quartile[2] = gradients
        .iter()
        .filter(|(grad, delta_x, delta_y)| delta_x < &0.0 && delta_y <= &0.0)
        .map(|x| *x)
        .collect();
    quartile[3] = gradients
        .iter()
        .filter(|(grad, delta_x, delta_y)| delta_x < &0.0 && delta_y > &0.0)
        .map(|x| *x)
        .collect();

    for i in 0..4 {
        quartile[i].sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        //println!("{:?}", quartile[i]);
        println!("{:?}", quartile[i].len());
    }

    // let unsorted = gradients.clone();
    // gradients.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    //println!("{:?}", gradients);

    quartile[3].reverse();
    println!("{:?}", quartile[3]);
    let index = gradients
        .iter()
        .position(|&x| x == quartile[3][50])
        .unwrap();
    println!("{:?}", points[index as usize]);
}
