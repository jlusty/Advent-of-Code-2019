use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
mod fix_surplus;

pub fn main() {
    let eq_vec = get_equations();
    let mut eq_map = HashMap::new();
    let mut pre_ore: Vec<String> = Vec::new();

    for eq in eq_vec {
        eq_map.insert(eq.product.1.clone(), (eq.product.0, eq.reactants.clone()));
        if eq.reactants[0].1 == "ORE" {
            pre_ore.push(eq.product.1);
        }
    }
    println!("{:?}", pre_ore);

    let final_reacts: Vec<(usize, String)> = eq_map.get("FUEL").unwrap().1.clone();
    let mut chems_needed = ChemsNeeded::new(final_reacts, pre_ore);

    while !chems_needed.is_empty() {
        //println!("{:?}", chems_needed);

        let (prod_quant, prod_name) = chems_needed.get_next();
        let (eq_quant, eq_reactants) = eq_map.get(&prod_name).unwrap().clone();

        let multiplier = (prod_quant + (eq_quant - 1)) / eq_quant;
        let needed_reactants: Vec<(usize, String)> = eq_reactants
            .into_iter()
            .map(|(quant, name)| (quant * multiplier, name))
            .collect();

        let needed_reactants = chems_needed.get_from_surplus(needed_reactants);

        //println!("{:?}", needed_reactants);

        chems_needed.remove_first(eq_quant * multiplier);
        chems_needed.add_reactants(needed_reactants);
    }

    //let surplus_fix = fix_surplus::main(chems_needed.surplus);
    //chems_needed.check_surplus(eq_map);
    chems_needed.clean_up_pre_ore();
    println!("{:?}", chems_needed.pre_ore_qty);
    chems_needed.extract_ore(eq_map);
    println!("{}", chems_needed.ore);
}

fn read_file(filename: String) -> Result<String, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    Ok(input)
}

#[derive(Debug)]
struct Equation {
    reactants: Vec<(usize, String)>,
    product: (usize, String),
}
impl Equation {
    fn new(reactants: Vec<(usize, String)>, product: (usize, String)) -> Equation {
        Equation { reactants, product }
    }
}

#[derive(Debug)]
struct ChemsNeeded {
    vec: Vec<(usize, String)>,
    surplus: Vec<(usize, String)>,
    pre_ore: Vec<String>,
    pre_ore_qty: Vec<(usize, String)>,
    ore: usize,
}
impl ChemsNeeded {
    fn new(reactants: Vec<(usize, String)>, pre_ore: Vec<String>) -> ChemsNeeded {
        let mut init = ChemsNeeded {
            vec: reactants,
            surplus: Vec::new(),
            pre_ore,
            pre_ore_qty: Vec::new(),
            ore: 0,
        };
        init.extract_pre_ore();
        init.vec.sort_by(|a, b| a.1.cmp(&b.1));
        init
    }

    fn get_from_surplus(
        &mut self,
        mut needed_reactants: Vec<(usize, String)>,
    ) -> Vec<(usize, String)> {
        let mut i = 0;
        let mut j = 0;
        while i < self.surplus.len() && j < needed_reactants.len() {
            if self.surplus[i].1 == needed_reactants[j].1 {
                match self.surplus[i].0.cmp(&needed_reactants[j].0) {
                    Ordering::Less | Ordering::Equal => {
                        needed_reactants[j].0 -= self.surplus[i].0;
                        self.surplus.remove(i);
                    }
                    Ordering::Greater => {
                        self.surplus[i].0 -= needed_reactants[j].0;
                        needed_reactants[j].0 = 0;
                        i += 1;
                    }
                }
                j += 1;
            } else {
                i += 1;
            }
        }
        needed_reactants
    }

    fn add_reactants(&mut self, reactants: Vec<(usize, String)>) {
        self.vec.extend(reactants);
        self.vec.sort_by(|a, b| a.1.cmp(&b.1));
        let mut i = 1;
        while i < self.vec.len() {
            if self.vec[i].1 == self.vec[i - 1].1 {
                self.vec[i - 1].0 += self.vec[i].0;
                self.vec.remove(i);
            } else {
                i += 1;
            }
        }
        self.extract_pre_ore();
    }

    fn extract_pre_ore(&mut self) {
        let mut i = 0;
        while i < self.vec.len() {
            let reactant = &self.vec[i];
            if self.pre_ore.contains(&reactant.1) {
                self.pre_ore_qty.push(reactant.clone());
                self.vec.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn clean_up_pre_ore(&mut self) {
        self.pre_ore_qty.sort_by(|a, b| a.1.cmp(&b.1));
        let mut i = 1;
        while i < self.pre_ore_qty.len() {
            if self.pre_ore_qty[i].1 == self.pre_ore_qty[i - 1].1 {
                self.pre_ore_qty[i - 1].0 += self.pre_ore_qty[i].0;
                self.pre_ore_qty.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn extract_ore(&mut self, eq_map: HashMap<String, (usize, Vec<(usize, String)>)>) {
        for reactant in self.pre_ore_qty.clone() {
            let mapping = eq_map.get(&reactant.1).unwrap();
            let prod_qty = mapping.1[0].0;
            let eq_qty = mapping.0;

            println!("{}< {}< {}", prod_qty, eq_qty, reactant.0);

            let multiplier = (reactant.0 + (eq_qty - 1)) / eq_qty;

            self.ore += prod_qty * multiplier;
        }
    }

    fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }

    fn get_next(&self) -> (usize, String) {
        self.vec.get(0).unwrap().clone()
    }

    fn remove_first(&mut self, amount: usize) {
        match amount.cmp(&self.vec[0].0) {
            Ordering::Greater => {
                self.surplus
                    .push((amount - self.vec[0].0, self.vec[0].1.clone()));
                self.vec.remove(0);
            }
            Ordering::Equal => {
                self.vec.remove(0);
            }
            Ordering::Less => panic!("Haven't got enough material"),
        };
        self.surplus.sort_by(|a, b| a.1.cmp(&b.1));
        let mut i = 1;
        while i < self.surplus.len() {
            if self.surplus[i].1 == self.surplus[i - 1].1 {
                self.surplus[i - 1].0 += self.surplus[i].0;
                self.surplus.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn check_surplus(&mut self, eq_map: HashMap<String, (usize, Vec<(usize, String)>)>) {
        for i in 0..self.surplus.len() {
            let r = self.surplus[i].clone();
            let mapping = eq_map.get(&r.1).unwrap().clone();
            if mapping.0 <= r.0 {
                println!(
                    "Have {} {} so can get {} * {:?}",
                    r.0,
                    r.1,
                    r.0 / mapping.0,
                    mapping.1
                );
            // println!("{:?}", r);
            //println!("{:?}", mapping);
            // println!("{}, {}", (mapping.1)[0].0, r.0);

            // if (mapping.1)[0].1 == "ORE" {
            //     self.ore -= (mapping.1)[0].0;
            // }
            } else {
                println!("Have {} {}", r.0, r.1);
            }
        }
    }
}

fn get_equations() -> Vec<Equation> {
    let input = read_file("./test.txt".to_string()).unwrap();
    let mut eq_vec: Vec<Equation> = Vec::new();

    for line in input.lines() {
        let mut iter_react_prod = line.split("=>");
        let react_str = iter_react_prod.next().unwrap();
        let prod_str = iter_react_prod.next().unwrap();

        let iter_react_str = react_str.split(",");
        let iter_num_name = iter_react_str.map(|s| s.trim().split(" ").collect());
        let reactants = iter_num_name
            .map(|r: Vec<&str>| (r[0].parse().unwrap(), r[1].to_owned()))
            .collect();

        let p: Vec<&str> = prod_str.trim().split(" ").collect();
        let product = (p[0].parse().unwrap(), p[1].to_owned());

        let eq = Equation::new(reactants, product);

        eq_vec.push(eq)
    }

    eq_vec
}
