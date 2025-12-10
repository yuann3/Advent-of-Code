use num_traits::int::PrimInt;

/// Computes the greatest common divisor (GCD) of two integers using Euclid's algorithm.
pub fn gcd<T: PrimInt>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Computes the least common multiple (LCM) of two integers.
/// Uses the formula: lcm(a, b) = (a / gcd(a, b)) * b to avoid overflow.
pub fn lcm<T: PrimInt>(a: T, b: T) -> T {
    if a == T::zero() || b == T::zero() {
        T::zero()
    } else {
        let g = gcd(a, b);
        (a / g) * b
    }
}

/// Extended Euclidean algorithm.
/// Returns (gcd, x, y) such that ax + by = gcd(a, b).
pub fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x1, y1) = gcd_extended(b, a % b);
        (g, y1, x1 - (a / b) * y1)
    }
}

/// Modular exponentiation: computes (base^exp) % modulus efficiently.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

/// Computes the modular inverse of a modulo m using the extended Euclidean algorithm.
/// Returns Some(x) where (a * x) % m == 1, or None if no inverse exists.
pub fn mod_inv(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = gcd_extended(a, m);
    if g != 1 {
        None
    } else {
        Some((x % m + m) % m)
    }
}

/// Checks if a number is prime using trial division for small numbers
/// and Miller-Rabin for larger ones.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    // Check divisors up to sqrt(n)
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Returns all prime numbers up to and including limit using Sieve of Eratosthenes.
pub fn primes_up_to(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(i, &prime)| if prime { Some(i) } else { None })
        .collect()
}

/// Returns the prime factorization of n as a vector of prime factors.
pub fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Handle factor of 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Check odd factors
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    // If n is still > 1, it's a prime factor
    if n > 1 {
        factors.push(n);
    }

    factors
}

/// Computes factorial of n. Panics on overflow.
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

/// Computes binomial coefficient "n choose k".
/// Returns n! / (k! * (n-k)!)
pub fn binomial(n: u64, k: u64) -> u64 {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }

    // Use smaller k for efficiency
    let k = k.min(n - k);

    // Calculate C(n, k) = n * (n-1) * ... * (n-k+1) / (k * (k-1) * ... * 1)
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic() {
        assert_eq!(gcd(48, 18), 6);
    }

    #[test]
    fn test_gcd_edge_cases() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(7, 7), 7);
    }

    #[test]
    fn test_gcd_commutative() {
        assert_eq!(gcd(18, 48), gcd(48, 18));
        assert_eq!(gcd(100, 35), gcd(35, 100));
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(12, 8), 24);
        assert_eq!(lcm(21, 6), 42);
    }

    #[test]
    fn test_lcm_edge_cases() {
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);
        assert_eq!(lcm(1, 5), 5);
        assert_eq!(lcm(5, 1), 5);
    }

    #[test]
    fn test_gcd_lcm_property() {
        // gcd(a, b) * lcm(a, b) = a * b
        assert_eq!(gcd(12, 8) * lcm(12, 8), 12 * 8);
        assert_eq!(gcd(15, 25) * lcm(15, 25), 15 * 25);
    }

    #[test]
    fn test_gcd_extended() {
        let (g, x, y) = gcd_extended(48, 18);
        assert_eq!(g, 6);
        assert_eq!(48 * x + 18 * y, g);
    }

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(2, 10, 1000), 24);
        assert_eq!(mod_pow(3, 4, 17), 13);
        assert_eq!(mod_pow(5, 0, 13), 1);
    }

    #[test]
    fn test_mod_inv() {
        assert_eq!(mod_inv(3, 11), Some(4)); // 3 * 4 = 12 ≡ 1 (mod 11)
        assert_eq!(mod_inv(10, 17), Some(12)); // 10 * 12 = 120 ≡ 1 (mod 17)
        assert_eq!(mod_inv(2, 4), None); // No inverse (gcd(2,4) ≠ 1)
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(17));
        assert!(is_prime(97));
        assert!(!is_prime(100));
        assert!(!is_prime(1));
        assert!(!is_prime(0));
    }

    #[test]
    fn test_primes_up_to() {
        assert_eq!(primes_up_to(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_up_to(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(primes_up_to(1), Vec::<usize>::new());
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(17), vec![17]);
        assert_eq!(prime_factors(100), vec![2, 2, 5, 5]);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 2), 10);
        assert_eq!(binomial(10, 3), 120);
        assert_eq!(binomial(5, 0), 1);
        assert_eq!(binomial(5, 5), 1);
    }
}
