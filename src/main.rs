use std::{collections::VecDeque, env, sync::mpsc::{self, Receiver}, thread}; 

fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    if args.len() == 1 {
        print_small_help();
        return;
    };

    let _ = args.pop_front();
    if args.len() == 1 {
        if args[0] == "--help".to_string() {
            print_help();
        } else if args[0] == "--version".to_string() {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        } else {
            list_factors(args);
        }
    } else {
        print_small_help();
    }
}

fn print_small_help() {
    println!("{} {}\nUse '{} --help' for help", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_NAME"))
}

fn print_help() {
    print_small_help();
    println!("
 --version -> Prints out name and Version
")
}

fn list_factors(mut args: VecDeque<String>) {
    let res = args.pop_front().unwrap().parse::<u64>();
    let divident: u64;
    match res {
        Ok(_) => divident = res.unwrap(),
        Err(_) => {println!("Please enter a valid u64"); return;},
    }

    if divident < 2 {
        println!("Please enter a number larger than 1");
        return;
    }

    let working_factors = factors_of(divident);

    println!();
    for i in 0..working_factors.len() {
        println!("{}", working_factors[i])
    }
}

pub fn factors_of(num:u64) -> Vec<u64> {
    let mut reces: Vec<Receiver<Vec<u64>>> = Vec::new();
    let mut factors: Vec<u64> = Vec::new();
    let mut threats = Vec::new();
    
    for i in 1..16 {
        let (tran, rece) = mpsc::channel();
        reces.push(rece);
        threats.push(thread::spawn( move || {
            tran.send(check(i, num, 15))
        }));
    };

    for rece in reces {
        factors.append(&mut rece.recv().unwrap());
    };

    for thr in threats {
        let join = thr.join();
        let _ = join.unwrap();
    };

    factors.sort();

    return factors;
}

fn check(start: u64, num: u64, increment: u64) -> Vec<u64> {
    let mut factor = start;
    let mut working_factors: Vec<u64> = Vec::new();
    let max_value = num as f64;
    let max_value = max_value.sqrt() as u64;
    while factor <= max_value {
        if num%factor == 0 {
            working_factors.push(factor);
            if factor != num/factor {
                working_factors.push(num/factor);
            }
        }
        factor += increment;
    }
    return working_factors;
}
