use std::collections::VecDeque;

pub struct PrimeSieve {
    max_poss_num: u128,
    primes: VecDeque<u128>,
}

impl PrimeSieve {
    pub fn new(max_poss_num: u128) -> PrimeSieve {
        PrimeSieve {
            max_poss_num,
            primes: get_primes(max_poss_num),
        }
    }

    pub fn is_prime(&self, num: u128) -> bool {
        if num > self.max_poss_num {
            panic!(
                "Number ({}) given to PrimeSieve::is_prime larger than maximum ({})!",
                num, self.max_poss_num
            );
        }
        self.primes.contains(&num)
    }
}

fn get_primes(max_poss_num: u128) -> VecDeque<u128> {
    let mut primes: VecDeque<u128> = (2..max_poss_num + 1).collect();
    let mut i = 0;
    while i < primes.len() {
        let curr_num = primes[i];
        primes.retain(|p| p % curr_num != 0 || *p == curr_num);
        i += 1;
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::PrimeSieve;

    #[test]
    fn is_prime_tests() {
        let prime_sieve = PrimeSieve::new(971);
        assert!(prime_sieve.is_prime(1) == false);
        assert!(prime_sieve.is_prime(0) == false);
        assert!(prime_sieve.is_prime(100) == false);
        assert!(prime_sieve.is_prime(6) == false);
        assert!(prime_sieve.is_prime(2));
        assert!(prime_sieve.is_prime(3));
        assert!(prime_sieve.is_prime(5));
        assert!(prime_sieve.is_prime(7));
        assert!(prime_sieve.is_prime(11));
        assert!(prime_sieve.is_prime(13));
        assert!(prime_sieve.is_prime(17));
        assert!(prime_sieve.is_prime(31));
        assert!(prime_sieve.is_prime(37));
        assert!(prime_sieve.is_prime(71));
        assert!(prime_sieve.is_prime(73));
        assert!(prime_sieve.is_prime(79));
        assert!(prime_sieve.is_prime(97));
        assert!(prime_sieve.is_prime(971));
        assert!(prime_sieve.is_prime(197));
        assert!(prime_sieve.is_prime(719));
    }
}
