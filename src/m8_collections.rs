#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_hashmap() {
        let person1: &str = "alice";
        let person2: &str = "bob";

        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert(person1, 55);
        results_hm.insert(person2, 51);

        let test_score: Option<&u32> = results_hm.get(person1);
        dbg!(test_score.unwrap());

        if results_hm.contains_key("alice") {
            dbg!("Alice is present!");
        }
    }

    #[test]
    fn test_hashset() {
        let mut results_hs: HashSet<&str> = HashSet::new();

        results_hs.insert("alice");
        results_hs.insert("bob");
        results_hs.insert("carol");

        if results_hs.contains("alice") {
            dbg!("Found alice");
        }
    }
}
