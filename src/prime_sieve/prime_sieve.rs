use std::collections::{BTreeMap, VecDeque};

// An implementation of the Sieve of Eratosthenes
pub struct PrimeSieve {
    max_poss_num: u128,
    primes: BTreeMap<u128, bool>,
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

        match self.primes.get(&num) {
            Some(is_pr) => return *is_pr,
            None => return false,
        }
    }
}

fn get_primes(max_poss_num: u128) -> BTreeMap<u128, bool> {
    let mut primes: BTreeMap<u128, bool> = BTreeMap::new();

    for i in 2..max_poss_num + 1 {
        primes.insert(i, true);
    }

    let mut found_primes: VecDeque<u128> = VecDeque::new();

    for (number, is_prime) in &mut primes {
        *is_prime = found_primes.iter().any(|pr| number % pr == 0) == false;

        if *is_prime {
            found_primes.push_back(*number);
        }
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
        assert!(prime_sieve.is_prime(41));
    }
}
