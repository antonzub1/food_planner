use serde_derive::{Serialize, Deserialize};


#[derive(Default, Serialize, Deserialize)]
pub struct DietPlan {
    pub target: Macros,
    pub meals: Vec<Meal>

}


impl DietPlan {
    pub fn new(protein: f32, carbs: f32, fats: f32) -> DietPlan {
        DietPlan {
            meals: Vec::new(),
            target:  Macros {
                protein,
                carbs,
                fats
            }
        }
    }

    pub fn total(&self) -> Macros {
        let result = self.meals.iter().fold(Macros::default(), |mut aggregate, item| {
            aggregate.add(&item.totals());
            aggregate
        });
        result
    }

    pub fn add(&mut self, meal: Meal) {
        self.meals.push(meal);
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct Meal {
    pub name: String,
    pub macros: Macros,
    pub weight: f32
}

impl Meal {
    pub fn totals(&self) -> Macros {
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

#[derive(Default, Serialize, Deserialize)]
pub struct Macros {
    pub protein: f32,
    pub carbs: f32,
    pub fats: f32
}

impl Macros {
    pub fn add(&mut self, rhs: &Macros) {
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


