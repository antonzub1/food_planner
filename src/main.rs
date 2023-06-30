mod types;

use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, about = "Save a meal and it's macros as a reference")]
    Add {
        #[arg(short, long, help = "Name of the meal")]
        name: Option<String>,
        #[arg(short, long, help = "Amount of protein in 100g")]
        protein: Option<f32>,
        #[arg(short, long, help = "Amount of carbs in 100g")]
        carbs: Option<f32>,
        #[arg(short, long, help = "Amount of fats in 100g")]
        fats: Option<f32>,
    },
    #[command(arg_required_else_help = true, about = "Add a meal to the ration. Must be added beforehand with 'add' command")]
    Track {
        #[arg(short, long, help = "Name of the meal to add to daily ration.")]
        name: Option<String>,
        #[arg(short, long, help = "Amount of the meal consumed")]
        amount: Option<f32>,
        #[arg(short, long, help = "Date of consumption. Optional. Falls back to current day if not provided")]
        date: Option<String>,
    }
}



fn main() {
    let args = CLI::parse();
}


#[cfg(test)]
mod tests {
    use super::*;
    use types::{Meal, Macros, DietPlan};

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
        diet.add(oatmeal);
        diet.add(turkey);
        assert!(diet.meals.len() == 2);
        assert!(diet.total() == expected);
    }
}
