use asymetric::*;

fn main() {
    println!("Enter a prime number :");
    let prime = read_positive_integer();
    if !is_prime(&prime) {
        panic!("Input must be a prime number.");
    }

    println!("Enter a generator");
    let generator = read_positive_integer();
    if !verify_conditions(&prime, &generator) {
        panic!("Respect the conditions ! ");
    }

    println!("Alice and Bob agree on public values: P = {prime}, G = {generator}");

    println!("Alice chooses  secret number :");
    let alice_secret = read_positive_integer();

    println!("Bob chooses  secret number :");
    let bob_secret = read_positive_integer();

    println!(
        "Alice's secret: a = {}\nBob's secret: b = {}",
        alice_secret, bob_secret
    );

    let alice_to_bob = generator.modpow(&alice_secret, &prime); // mod pow hadi takhdem b l mowafa9at bach teviti ta7seb l exposant direct ki ytih kbir
    let bob_to_alice = generator.modpow(&bob_secret, &prime);

    println!("Alice sends to Bob: {}", alice_to_bob);
    println!("Bob sends to Alice: {}", bob_to_alice);

    let alice_shared_key = bob_to_alice.modpow(&alice_secret, &prime); 
    let bob_shared_key = alice_to_bob.modpow(&bob_secret, &prime);

    println!("Shared secret key by Alice: {}", alice_shared_key);
    println!("Shared secret key by Bob: {}", bob_shared_key);

    if alice_shared_key != bob_shared_key {
        println!("Oops! Shared keys do NOT match.");
    }
}
