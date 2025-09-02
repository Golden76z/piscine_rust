use json::{JsonValue, object};

pub struct Food {
    pub name: String,
    pub calories: (String, String),
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn parse_calories(s: &str) -> f64 {
    s.trim_end_matches(|c: char| !c.is_numeric() && c != '.')
        .parse::<f64>()
        .unwrap_or(0.0)
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;

    for item in foods {
        let portion = item.nbr_of_portions;

        // Use the kcal value (second in the tuple)
        let cal = parse_calories(&item.calories.1);

        total_cals += cal * portion;
        total_fats += item.fats * portion;
        total_carbs += item.carbs * portion;
        total_proteins += item.proteins * portion;
    }

    fn round_to_2_decimals(x: f64) -> f64 {
        let rounded = (x * 100.0).round() / 100.0;
        if (rounded * 10.0).fract() == 0.0 {
            (rounded * 10.0).round() / 10.0 // remove trailing zero
        } else {
            rounded
        }
    }

    object! {
        "cals" => round_to_2_decimals(total_cals),
        "fats" => round_to_2_decimals(total_fats),
        "carbs" => round_to_2_decimals(total_carbs),
        "proteins" => round_to_2_decimals(total_proteins)
    }
}
