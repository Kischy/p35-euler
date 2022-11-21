use crate::{
    circular_prime_number::circular_prime_number::is_circular_prime,
    prime_sieve::prime_sieve::PrimeSieve,
};

mod circular_prime_number;
mod prime_sieve;

fn main() {
    let problem_number = 35;
    let mut answer_p35 = 0;

    let max = 1000000;
    let prime_sieve = PrimeSieve::new(max);

    for num in 2..max {
        if is_circular_prime(num, &prime_sieve) {
            answer_p35 += 1;
        }
    }

    println!(
        "The answer to problem {} of project Euler is {}.",
        problem_number, answer_p35
    );
}
