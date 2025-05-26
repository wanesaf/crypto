use indice_coincidence::{calculate_coincidence, create_collection}; 
fn main() {
    let word = String::from("haha");
    let collection = create_collection(&word);
    let result = calculate_coincidence(collection, word.len());
    println!("{}",result);
}

