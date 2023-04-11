use std::ops::{Add, Sub};
struct Diet {
    meals: Vec<Meal>

}

impl Diet {
    fn total(&self) -> Macros {
        let mut result = Macros::default();
        // TODO
        result
    }
}

struct Meal {
    macros: Macros
}

#[derive(Default)]
struct Macros {
    protein: f32,
    carbs: f32,
    fat: f32
}

impl Add for Macros {
    type Output = Macros;

    fn add(self, rhs: Macros) -> Self::Output {
        Macros {
            protein: self.protein + rhs.protein,
            carbs: self.carbs + rhs.carbs,
            fat: self.fat + rhs.fat
        }
    }
}

impl Add for Meal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            macros: self.macros + rhs.macros
        }
    }
}

fn main() {
    println!("Hello, world!");
}
