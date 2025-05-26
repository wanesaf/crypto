use asymetric::{is_prime, read_positive_integer, verify_elgamal_generator};
use num::{BigUint, One};
use num_bigint::ToBigUint;
use std::io::{self, Write};
fn main() {
    let p = loop {
        print!("Donner un nombre premier svp: ");
        io::stdout().flush().unwrap();
        let p = read_positive_integer();
        if is_prime(&p) {
            break p;
        }
    };

    let g: BigUint = loop {
        print!("choisir g Є (0...p-1) premier avec p,avec g^z ≠ 1 mod p ∀z Є (1...p-2): ");
        io::stdout().flush().unwrap();
        let g = read_positive_integer();
        if verify_elgamal_generator(&p, &g) {
            break g;
        }
    };

    let s = loop {
        print!("choisir s tq s Є (1...p-2): ");
        io::stdout().flush().unwrap();
        let s = read_positive_integer();
        if s >= One::one() && s <= &p - 2_u32.to_biguint().unwrap() {
            break s;
        }
    };

    let y = g.modpow(&s, &p);
    println!("la clé publique est : ({y},{g},{p})");
    println!("la clé privé est : ({s})");

    print!("choisir le message a chiffrer: ");
    io::stdout().flush().unwrap();
    let m = read_positive_integer();

    print!("choisir la clé k  : ");
    io::stdout().flush().unwrap();
    let k = read_positive_integer();

    let c1 = g.modpow(&k,&p);
    let c2 = (m * y.modpow(&k,&p)).modpow(&One::one(),&p); 

    println!("le message chiffré est : ({c1},{c2})"); 

    let r = c1.modpow(&s,&p) ; 

    let m = (c2 * r.modinv(&p).unwrap()).modpow(&One::one(),&p) ; 

    println!("le message dechiffré est : {m}") ; 
}
