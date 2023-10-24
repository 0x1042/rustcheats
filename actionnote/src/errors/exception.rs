use std::{
    any::type_name,
    fs::File,
    io::{Error, Read},
    mem::{align_of, size_of},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn run() -> Result<Vec<Point>, Error> {
    let mut f = File::open("")?;
    let meta = f.metadata()?;
    println!("meta: {:?}", meta);
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let points: Vec<Point> = serde_json::from_str(&buf)?;

    Ok(points)
}

fn dbg_size<T>() {
    println!(
        "{} size is {}, align {}",
        type_name::<T>(),
        size_of::<T>(),
        align_of::<T>()
    );
}

#[cfg(test)]
mod tests {
    use super::dbg_size;

    enum Demo {
        A,
        B(String),
        C(i64),
        D(std::io::Error),
    }

    #[test]
    fn test_size() {
        dbg_size::<Demo>();
    }
}
