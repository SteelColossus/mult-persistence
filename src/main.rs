use std::io;
use std::process;

use rayon::prelude::*;

fn main() {
    let mut line = String::new();

    println!("Enter a min length:");
    io::stdin()
        .read_line(&mut line)
        .expect("Unable to read line");
    let min: usize = line.trim().parse().unwrap_or_else(|err| {
        eprintln!("Error: {}. Please enter a positive number above zero.", err);
        exit();
    });

    if min == 0 {
        eprintln!("Please enter a positive number above zero.");
        exit();
    }

    line = String::new();

    println!("Enter a max length:");
    io::stdin()
        .read_line(&mut line)
        .expect("Unable to read line");
    let max: usize = line.trim().parse().unwrap_or_else(|err| {
        eprintln!("Error: {}. Please enter a positive number above zero.", err);
        exit();
    });

    if max == 0 {
        eprintln!("Please enter a positive number above zero.");
        exit();
    } else if min > max {
        eprintln!("Please enter a minimum length that is less than the maximum length.");
        exit();
    }

    (min..=max).into_par_iter().for_each(|x| {
        println!("{} digits slice started", x);
        let (digits, res_digit, n_times) = mult_persistence::calc_slice(x);
        println!(
            "{} digits slice finished: {:?} -> {} in {} steps{}",
            x,
            digits,
            res_digit,
            n_times,
            if n_times >= 11 { " ***" } else { "" }
        );
    });

    println!("All slices calculated.");
}

fn exit() -> ! {
    process::exit(1);
}
