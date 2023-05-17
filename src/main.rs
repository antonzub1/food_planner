mod types;

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Add(AddArgs),
}


#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true)]
struct AddArgs {
    #[arg(short, long)]
     meal: Option<String>
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
        diet.add(&oatmeal);
        diet.add(&turkey);
        assert!(diet.meals.len() == 2);
        assert!(diet.total() == expected);
    }
}
