#[derive(Debug)]
struct Sheep<'c, 'd> {
    age: &'c u32,
    name: &'d str,
}

impl<'c, 'd> Sheep<'c, 'd> {
    /// Returns the get age of this [`Sheep`].
    pub fn get_age(&self) -> u32 {
        *self.age
    }

    /// Returns the get name of this [`Sheep`].
    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::Sheep;
    #[test]
    fn test_lt() {
        let age = 5;
        let name = "dollor";

        let sheep = Sheep {
            age: &age,
            name: name,
        };

        println!("name is {:?}", sheep.get_name());
        println!("age is {:?}", sheep.get_age());

        println!("sheep :{:?}", sheep);
    }
}
