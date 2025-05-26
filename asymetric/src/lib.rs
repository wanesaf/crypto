use num::Integer;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use std::io;

pub fn read_positive_integer() -> BigUint {
    let stdin = io::stdin();
    loop {
        let mut buffer = String::new();
        if stdin.read_line(&mut buffer).is_ok() {
            match buffer.trim().parse::<BigUint>() {
                Ok(n) if n > Zero::zero() => return n, // zero zero hadi takhdem b generic , tsma comme hna rah nakhdmo bigUint rah yearef lcompiler bli had zero du meme type
                _ => println!("Please enter a valid positive integer."),
            }
        } else {
            println!("Failed to read input.");
        }
    }
}

pub fn verify_conditions(prime: &BigUint, g: &BigUint) -> bool {
    *g > One::one() && g < prime
}

pub fn is_prime(number: &BigUint) -> bool {
    let two = 2_u32.to_biguint().unwrap(); // 2_u32 is same as (2 as u32 ) , we use this because biguint works on u32s
    let three = 3_u32.to_biguint().unwrap();

    match number {
        n if n < &two => false, // no need to dereference , comparing refs working fine (because compiler deref it automatically)
        n if n == &two || n == &three => true,
        n if n % &two == Zero::zero() || n % &three == Zero::zero() => false,
        _ => {
            let mut i = 5_u32.to_biguint().unwrap();
            let mut step = 2_u32;
            while &i * &i <= *number {
                // u must borrow it biguint hadi tmovi kolech liha 
                if number % &i == Zero::zero() {
                    // kona 9adrin nakhdmo b 0_u32 bsa7 rah tch9a tconvertih l3fsa kbira bzaf kima biguint , tsma zero trait 9owa hna
                    return false;
                }
                i += step.to_biguint().unwrap();
                step = 6 - step;
            }
            true
        }
    }
}

pub fn verify_elgamal_generator(p: &BigUint, g: &BigUint) -> bool {
    let one: BigUint = One::one();
    let two = 2_u32.to_biguint().unwrap();
    if *g >= Zero::zero() && *g <= p - &one {
        if g.gcd(&p) == one {
            let mut z: BigUint = one.clone();
            while z < p - &two {
                if g.modpow(&z, &p) == one {
                    println!("g^z doit etre  ≠ 1 mod p  pour tous z Є (1...p-2).");
                    return false;
                }
                z += &one; 
            }
        } else {
            println!("p et g doivent etres premiers entre eux");
            return false;
        }
    } else {
        println!("g doit etre Є (0...p-1)");
        return false;
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_check_true() {
        assert_eq!(is_prime(&47_u32.to_biguint().unwrap()), true);
    }

    #[test]
    fn test_prime_check_false() {
        assert_eq!(is_prime(&65_u32.to_biguint().unwrap()), false);
    }

    #[test]
    fn test_verify_conditions_pass() {
        let prime = 53_u32.to_biguint().unwrap();
        let g = 52_u32.to_biguint().unwrap();
        assert_eq!(verify_conditions(&prime, &g), true);
    }

    #[test]
    fn test_verify_conditions_fail() {
        let prime = 53_u32.to_biguint().unwrap();
        let g = 54_u32.to_biguint().unwrap();
        assert_eq!(verify_conditions(&prime, &g), false);
    }
}
