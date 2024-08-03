#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn tests_hashmap() {
        let person_1 = "alice";
        let person_2 = "bob";

        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 51);

        let test_score = results_hm.get(person_1);
        let score = test_score.unwrap();
        assert_eq!(*score, 55);
    }
}
