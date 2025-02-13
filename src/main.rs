use std::{env, sync::mpsc::{self, Receiver}, thread}; 

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Find Dovisors\nversion 0.1\nUse: find_divisor [u64] ([path])");
        return;
    };
    
    let res = args[1].parse::<u64>();
    let divident: u64;
    match res {
        Ok(_) => divident = res.unwrap(),
        Err(_) => {println!("Please enter valid u64"); return;},
    }

    if divident < 2 {
        println!("Please enter a number that is larger than 1");
        return;
    }

    let mut reces: Vec<Receiver<Vec<u64>>> = Vec::new();
    let mut working_divisors: Vec<u64> = Vec::new();
    let mut threats = Vec::new();
    
    for i in 1..16 {
        let (tran, rece) = mpsc::channel();
        reces.push(rece);
        threats.push(thread::spawn( move || {
            tran.send(check(i, divident, 15))
        }));
    };

    for rece in reces {
        working_divisors.append(&mut rece.recv().unwrap());
    };

    for thr in threats {
        let join = thr.join();
        let _ = join.unwrap();
    };

    working_divisors.sort();
    working_divisors.push(divident);

    println!();
    for i in 0..working_divisors.len() {
        println!("{}", working_divisors[i])
    }
}

fn check(start: u64, divident: u64, increment: u64) -> Vec<u64> {
    let mut divisor = start;
    let mut working_divisors: Vec<u64> = Vec::new();
    let max_value = divident as f64;
    let max_value = max_value.sqrt() as u64;
    while divisor <= max_value {
        if divident%divisor == 0 {
            println!("{}", divisor);
            working_divisors.push(divisor);
        }
        divisor += increment;
    }
    return working_divisors;
}
