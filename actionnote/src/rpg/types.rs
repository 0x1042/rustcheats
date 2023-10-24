use rand::Rng;

#[derive(Debug)]
pub enum Thing {
    Sword,
    Trinket,
}

pub trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchanter(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);
        if spell_is_successful {
            println!("The {:?} glows brightly", thing);
        } else {
            println!(
                "The {:?} fizzes, then turns into a worthless trinket",
                thing
            );
            *thing = Thing::Trinket {};
        }
    }
}

#[derive(Debug)]
pub struct Human {}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.5
    }
}

#[derive(Debug)]
pub struct Elve {}

impl Enchanter for Elve {
    fn competency(&self) -> f64 {
        0.95
    }
}

#[derive(Debug)]
pub struct Dwarve {}

impl Enchanter for Dwarve {
    fn competency(&self) -> f64 {
        0.8
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use super::{Dwarve, Elve, Enchanter, Human, Thing};

    #[test]
    fn test_rpg() {
        let mut it = Thing::Sword;

        let d = Dwarve {};
        let e = Elve {};
        let h = Human {};

        let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];

        let spellcaster = party.choose(&mut rand::thread_rng()).unwrap();

        spellcaster.enchanter(&mut it);
    }
}
