use crate::prime_sieve::prime_sieve::PrimeSieve;
mod rotating_number;

pub fn is_circular_prime(num: u128, primes: &PrimeSieve) -> bool {
    for poss_prime in rotating_number::RotatingNumber::new(num) {
        if primes.is_prime(poss_prime) == false {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_circular_prime;
    use super::PrimeSieve;

    #[test]
    fn is_rotating_prime_positive_examples() {
        let primes = PrimeSieve::new(1000);
        assert!(is_circular_prime(197, &primes));
        assert!(is_circular_prime(971, &primes));
        assert!(is_circular_prime(719, &primes));
        assert!(is_circular_prime(2, &primes));
        assert!(is_circular_prime(3, &primes));
        assert!(is_circular_prime(5, &primes));
        assert!(is_circular_prime(7, &primes));
        assert!(is_circular_prime(11, &primes));
        assert!(is_circular_prime(13, &primes));
        assert!(is_circular_prime(17, &primes));
        assert!(is_circular_prime(31, &primes));
        assert!(is_circular_prime(37, &primes));
        assert!(is_circular_prime(71, &primes));
        assert!(is_circular_prime(73, &primes));
        assert!(is_circular_prime(79, &primes));
        assert!(is_circular_prime(97, &primes));
    }

    #[test]
    fn is_rotating_prime_negative_examples() {
        let primes = PrimeSieve::new(1000);
        assert!(is_circular_prime(1, &primes) == false);
        assert!(is_circular_prime(80, &primes) == false);
        assert!(is_circular_prime(41, &primes) == false);
    }
}
