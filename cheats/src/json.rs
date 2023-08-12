use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub name: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tracing::info;
    use tracing_test::traced_test;

    use crate::json::Animal;

    #[traced_test]
    #[test]
    fn struct_to_json() {
        let tom = Animal {
            name: "tom".to_string(),
            age: 20,
        };
        let jstr = serde_json::to_string(&tom).unwrap();
        assert!(jstr.len() > 0);
    }

    #[test]
    #[traced_test]
    fn struct_from_json() {
        let jstr = "{\"name\":\"tom\",\"age\":20}";

        let cat: Animal = serde_json::from_str(jstr).unwrap();
        assert_eq!(cat.age, 20);
    }

    #[test]
    #[traced_test]
    fn read_json_file() {
        let dir = std::env::current_dir().unwrap();
        info!("=================current dir {:?} ==================", &dir);
        let file = dir.join("../testdata/animals.json");
        info!(" cur file {:?}", &file);
        let arr_str = std::fs::read_to_string(file).unwrap();
        let animals: Vec<Animal> = serde_json::from_str(&arr_str).unwrap();
        assert_eq!(animals.len(), 2);
    }

    #[test]
    #[traced_test]
    fn map_to_json() {
        let mut db = HashMap::new();
        db.insert(
            "tom",
            Animal {
                name: "tom".to_owned(),
                age: 10,
            },
        );
        db.insert(
            "jerry",
            Animal {
                name: "jerry".to_owned(),
                age: 12,
            },
        );
        let jstr = serde_json::to_string(&db).unwrap();
        assert!(jstr.len() > 0);
    }

    #[test]
    #[traced_test]
    fn json_to_map() {
        let jstr = "{\"name\":\"tom\",\"age\":20}";

        let db: HashMap<String, serde_json::Value> = serde_json::from_str(jstr).unwrap();

        assert_eq!(db.len(), 2);
    }
}
