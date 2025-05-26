use std::collections::HashMap;
pub fn create_collection<'a>(word: &'a str) -> HashMap<char, u32> {
    let mut collection: HashMap<char, u32> = HashMap::new();
    let chars = word.chars();

    for c in chars {
        let count = collection.entry(c).or_insert(0);
        *count += 1;
    }

    collection
}

pub fn calculate_coincidence(h: HashMap<char, u32>, len: usize) -> f32 {
    if len == 0 {
        return 0.0;
    }
    let mut result = 0;
    for v in h.values() {
        result += v * (v - 1);
    }
    (result as f32) / ((len * (len - 1)) as f32)
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn should_work () {
        let word  =  String::from("BONJOURBONSOIRBONNEJOURNEE");
        let collection = create_collection(&word);
        let result = calculate_coincidence(collection,word.len());
        assert_eq!(result,0.11076923);
    }

    #[test]
    fn should_work_also () {
        let word  =  String::from("");
        let collection = create_collection(&word);
        let result = calculate_coincidence(collection,word.len());
        assert_eq!(result,0.0);
    }
}
