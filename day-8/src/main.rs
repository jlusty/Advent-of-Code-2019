use std::error::Error;
use std::fs;

fn main() {
    // let width = 2;
    // let height = 2;
    let width = 25;
    let height = 6;
    let layer_size = width * height;
    let input = get_string();

    //part_1(&input, layer_size);
    part_2(&input, layer_size, width);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

fn get_string() -> String {
    read_file("./input.txt".to_string()).unwrap()
}

// fn get_vector() -> Vec<usize> {
//     let input = read_file("./test.txt".to_string()).unwrap();
//     input.split(",").map(|x| x.parse().unwrap()).collect()
// }

fn part_1(input: &str, layer_size: usize) {
    let mut fewest_zeros: Option<usize> = None;
    let mut ones_twos = 0;
    let mut z = input.chars().peekable();
    while z.peek().is_some() {
        let layer: String = z.by_ref().take(layer_size).collect();

        let num_zeros = layer.chars().filter(|c| c == &'0').count();
        if fewest_zeros.is_none() || num_zeros < fewest_zeros.unwrap() {
            fewest_zeros = Some(num_zeros);
            ones_twos = layer.chars().filter(|c| c == &'1').count()
                * layer.chars().filter(|c| c == &'2').count();
        }
    }
    println!("{}", ones_twos);
}

fn part_2(input: &str, layer_size: usize, width: usize) {
    let mut image = "2".repeat(layer_size);
    let mut z = input.chars().peekable();
    while z.peek().is_some() {
        let layer: String = z.by_ref().take(layer_size).collect();

        image = image
            .chars()
            .zip(layer.chars())
            .map(|(img, lay)| if img == '2' { lay } else { img })
            .collect();
    }

    let mut z = image
        .chars()
        .map(|c| if c == '0' { ' ' } else { '□' })
        .peekable();
    while z.peek().is_some() {
        let row: String = z.by_ref().take(width).collect();

        println!("{}", row);
    }
    //println!("{}", image);
}
