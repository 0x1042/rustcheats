use std::collections::HashMap;
use std::hash::Hash;

fn print_vec<T>(vec: &[T])
where
    T: std::fmt::Debug,
{
    for (idx, val) in vec.iter().enumerate() {
        println!("{}@{:?}", idx, val);
    }
}

pub struct Pair<X, Y>(X, Y);

fn pair_to_hashmap<X, Y>(pairs: &[Pair<X, Y>]) -> HashMap<X, Y>
where
    X: Eq + Clone + Hash,
    Y: Clone,
{
    let mut db = HashMap::with_capacity(pairs.len());
    for pair in pairs.iter() {
        db.insert(pair.0.clone(), pair.1.clone());
    }
    db
}

#[cfg(test)]
mod tests {
    use crate::traits::{pair_to_hashmap, print_vec, Pair};

    #[test]
    fn test_print_vec() {
        let v1 = vec![1, 2, 3, 4, 5, 6];
        print_vec(&v1);

        let v2 = vec!["hello", "world", "hello1", "world2"];
        print_vec(&v2);
    }

    #[test]
    fn test_convert() {
        let p1 = Pair { 0: 1, 1: "1" };
        let p2 = Pair { 0: 2, 1: "2" };
        let p3 = Pair { 0: 3, 1: "3" };
        let ps = vec![p1, p2, p3];
        let db = pair_to_hashmap(&ps);
        println!("db {:?}", db);
    }
}
