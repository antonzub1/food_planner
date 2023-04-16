struct DietPlan<'a> {
    target: Macros,
    meals: Vec<&'a Meal>

}


impl<'a> DietPlan<'a> {
    fn new(protein: f32, carbs: f32, fats: f32) -> DietPlan<'a> {
        DietPlan {
            meals: Vec::new(),
            target:  Macros {
                protein: protein,
                carbs: carbs,
                fats
            }
        }
    }

    fn total(&self) -> Macros {
        let mut result = Macros::default();
        for meal in &self.meals {
            result.add(&meal.totals());
        }
        result
    }

    fn add(&mut self, meal: &'a Meal) {
        self.meals.push(meal);
    }
}

#[derive(Default)]
struct Meal {
    name: String,
    macros: Macros,
    weight: f32
}

impl Meal {
    fn totals(&self) -> Macros {
        let multiplier = self.weight / 100.0;
        Macros {
            protein: self.macros.protein * multiplier,
            carbs: self.macros.carbs * multiplier,
            fats: self.macros.fats * multiplier
        }
    }
}

impl Meal {
    fn add(mut self, rhs: &Meal) {
        self.macros.add(&rhs.macros);
    }
}

#[derive(Default)]
struct Macros {
    protein: f32,
    carbs: f32,
    fats: f32
}

impl Macros {
    fn add(&mut self, rhs: &Macros) {
        self.protein = self.protein + rhs.protein;
        self.carbs = self.carbs + rhs.carbs;
        self.fats = self.fats + rhs.fats;
    }
}

impl PartialEq for Macros {
    fn eq(&self, other: &Macros) -> bool {
        self.protein == other.protein &&
        self.carbs == other.carbs &&
        self.fats == other.fats
    }
}


fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diet_total_calc() {
        let mut diet = DietPlan::new(0.0, 0.0, 0.0);
        let expected = Macros {
            protein: 42.5,
            carbs: 58.7,
            fats: 14.4
        };
        let oatmeal = Meal {
            name: String::from("Oatmeal"),
            weight: 100.,
            macros: Macros {
                protein: 13.5,
                carbs: 58.7,
                fats: 7.
            }
        };
        let turkey = Meal {
            name: String::from("Turkey"),
            weight: 100.,
            macros: Macros {
                protein: 29.,
                carbs: 0.,
                fats: 7.4
            }
        };
        diet.add(&oatmeal);
        diet.add(&turkey);
        assert!(diet.meals.len() == 2);
        assert!(diet.total() == expected);
    }
}
