use std::{
    collections::VecDeque,
    env,
    sync::mpsc::{self, Receiver},
    thread,
};

fn main() {
    let mut args: VecDeque<String> = env::args().collect();
    if args.len() == 1 {
        print_small_help();
        return;
    };

    let _ = args.pop_front();
    if args[0] == "help".to_string() {
        print_help();
        return;
    } else if args[0] == "version".to_string() {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.len() == 1 {
        list_factors(args);
    } else {
        list_common_factors(args);
    }
}

fn print_small_help() {
    println!(
        "{} {}\nUse '{} help' for help",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_NAME")
    )
}

fn print_help() {
    print_small_help();
    println!(
        "
 version -> Prints out name and Version
 help    -> Prints out this help

Usage:

 {} [number]
 {} [number] [number] ... 
",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_NAME")
    )
}

// Function getting called if a single number is to be factorized
fn list_factors(mut args: VecDeque<String>) {
    let res = args.pop_front().unwrap().parse::<u64>();
    let num: u64;
    match res {
        Ok(value) => num = value,
        Err(_) => {
            println!("Please enter a valid u64");
            return;
        }
    }

    if num < 2 {
        println!("Please enter a number larger than 1");
        return;
    }

    let working_factors = factors_of(num, 8);

    println!();
    for i in 0..working_factors.len() {
        println!("{}", working_factors[i])
    }
}

// List getting called to list common factors between given numbers
fn list_common_factors(args: VecDeque<String>) {
    let mut nums: Vec<u64> = Vec::new();

    for arg in args {
        let res = arg.parse::<u64>();
        match res {
            Ok(value) => nums.push(value),
            Err(_) => {
                println!("{} is not a in range of u64", arg)
            }
        }
    }

    let common_factors = common_factors_of(nums);
    for common_factor in common_factors {
        println!("{}", common_factor)
    }
}

// Function for finding common factors between two numbers
fn common_factors_of(nums: Vec<u64>) -> Vec<u64> {
    let mut reces: Vec<Receiver<Vec<u64>>> = Vec::new();
    let mut threats = Vec::new();
    let mut factor_lists: Vec<Vec<u64>> = Vec::new();

    // Starts a thread per number, finding their factors
    for num in nums {
        let (tran, rece) = mpsc::channel();
        reces.push(rece);
        threats.push(thread::spawn(move || tran.send(factors_of(num, 4))));
    }

    // Receives the vectors containing the factors
    for rece in reces {
        factor_lists.push(rece.recv().unwrap());
    }

    // Kills the threads.
    for thr in threats {
        let join = thr.join();
        let _ = join.unwrap();
    }

    // Compares the factor lists
    let mut factor_list: Vec<u64> = factor_lists.pop().unwrap();
    for fl in factor_lists {
        // factor list retains all factors which are contained in fl
        factor_list.retain(|&factor| fl.contains(&factor));
    }
    factor_list.shrink_to_fit();
    return factor_list;
}

// Function for finding factors of a number
fn factors_of(num: u64, t: u64) -> Vec<u64> {
    let mut reces: Vec<Receiver<Vec<u64>>> = Vec::new();
    let mut factors: Vec<u64> = Vec::new();
    let mut threats = Vec::new();

    for i in 1..t + 1 {
        let (tran, rece) = mpsc::channel();
        reces.push(rece);
        threats.push(thread::spawn(move || tran.send(check(i, num, t))));
    }

    for rece in reces {
        factors.append(&mut rece.recv().unwrap());
    }

    for thr in threats {
        let join = thr.join();
        let _ = join.unwrap();
    }

    factors.sort();

    return factors;
}

// Function getting called checking every [increment] number between [start] and the sqrt of [num] for being a factor of [num].
fn check(start: u64, num: u64, increment: u64) -> Vec<u64> {
    let mut factor = start;
    let mut working_factors: Vec<u64> = Vec::new();
    let max_value = num as f64;
    let max_value = max_value.sqrt() as u64;
    while !{ factor > max_value } {
        if num % factor == 0 {
            working_factors.push(factor);
            if factor != num / factor {
                working_factors.push(num / factor);
            }
        }
        factor += increment;
    }
    return working_factors;
}
