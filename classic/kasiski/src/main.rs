

fn main() {
    let mut key = String::new();
    match kasiski::read_inputs("key", &mut key) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    };

    let mut plain = String::new();
    match kasiski::read_inputs("plain", &mut plain) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e)
        }
    };

    kasiski::freq_analyse(&key, &plain);
}
