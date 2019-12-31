use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() {
    part_2();
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn part_1() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut obj_map = HashMap::new();

    for line in input.lines() {
        let mut line_iter = line.split(")");
        let inner_obj = line_iter.next().unwrap();
        let outer_obj = line_iter.next().unwrap();
        obj_map.insert(outer_obj, inner_obj);
    }
    //println!("{:?}", obj_map);

    let mut total_orbit_depth = 0;
    for key in obj_map.keys() {
        let mut orbit_depth = 0;
        let mut inner_obj = key;
        while inner_obj != &"COM" {
            inner_obj = obj_map.get(inner_obj).unwrap();
            orbit_depth += 1;
        }
        total_orbit_depth += orbit_depth;
    }

    println!("{}", total_orbit_depth);
}

fn part_2() {
    let input = read_file("./input.txt".to_string()).unwrap();

    let mut obj_map = HashMap::new();

    for line in input.lines() {
        let mut line_iter = line.split(")");
        let inner_obj = line_iter.next().unwrap();
        let outer_obj = line_iter.next().unwrap();
        obj_map.insert(outer_obj, inner_obj);
    }
    //println!("{:?}", obj_map);

    let my_orbit = get_orbit(&obj_map, &"YOU");
    let santa_orbit = get_orbit(&obj_map, &"SAN");
    let mut i = 1;
    while my_orbit[my_orbit.len() - i] == santa_orbit[santa_orbit.len() - i] {
        i += 1;
        //println!("{}", my_orbit[my_orbit.len() - i]);
    }

    let transfer_dist = (my_orbit.len() - i) + (santa_orbit.len() - i);
    println!("{}", transfer_dist);
}

fn get_orbit<'a>(obj_map: &HashMap<&'a str, &'a str>, inner_obj: &'static str) -> Vec<&'a str> {
    let mut orbit: Vec<&str> = Vec::new();
    let mut inner_obj = inner_obj.clone();
    orbit.push(inner_obj);
    while inner_obj != "COM" {
        inner_obj = obj_map.get(inner_obj).unwrap();
        orbit.push(inner_obj);
    }
    orbit
}
