use std::io::{self,Write};
use asymetric::{is_prime, read_positive_integer};
use num_bigint::BigUint;
use num_traits::One;
use num::Integer ; 
fn main() {
    let (p,q) = loop {
        println!("Please choose two different prime numbers");
        print!("first: ");
        io::stdout().flush().unwrap();
        let p = read_positive_integer();
        print!("second: ");
        io::stdout().flush().unwrap();
        let q = read_positive_integer();
        if is_prime(&p) && is_prime(&q) {
            if p!=q {
                break (p,q)
            }else {
                println!("two different numbers please ! ");
            }
        }else {
            println!("prime numbers please");
        }
    }; 
    let n = &p * &q ; 
    let one : BigUint  = One::one() ; 
    let fi_n = (&p - &one) * (&q - &one); 
    println!("Φ(n) is {fi_n}");
    let e = loop {
        print!("choose a number that is prime with Φ(n): ") ;
        io::stdout().flush().unwrap(); 
        let e = read_positive_integer() ; 
        if e.gcd(&fi_n) == one  {
            break e 
        }
    };

    let d = e.modinv(&fi_n) ; 
    let d = match d {
        Some(i) => i ,
        None => panic!("inverse doesnt exist")
    };
    println!("the encryption exposant is : {e}");
    println!("the decryption exposant is : {d}"); 
    println!("the public key is ({n},{e})");
    println!("the private key is ({n},{d})");

    let m = loop { // this is for the plain text
        print!("Enter the plain text: ");
        io::stdout().flush().unwrap();
        let m = read_positive_integer() ; 
        if m < n  {
            break m
        }else {
            println!("M must be less than n ") ; 
        }
    };

    let c = m.modpow(&e,&n) ; 
    println!("the encrypted message is {c}") ;

    let m = c.modpow(&d, &n) ; 
    println!("the decrypted message is {m}");  
}
