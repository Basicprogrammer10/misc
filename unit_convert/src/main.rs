mod units;

use dialoguer::{theme::ColorfulTheme, FuzzySelect};

fn main() {
    let unit_space_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Unit Space")
        .default(0)
        .items(units::UNIT_TYPES)
        .interact()
        .unwrap();
    let unit_space = units::UNIT_CONVERSIONS[unit_space_index];

    let from_unit_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("From")
        .default(0)
        .items(unit_space)
        .interact()
        .unwrap();

    let to_unit_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("From")
        .default(0)
        .items(unit_space)
        .interact()
        .unwrap();

    let value = dialoguer::Input::<f64>::new()
        .with_prompt("Value")
        .interact()
        .unwrap();

    let from_unit = unit_space[from_unit_index];
    let to_unit = unit_space[to_unit_index];
    let base = from_unit.to_base(&value);
    let result = to_unit.from_base(&base);

    println!("{} {} = {} {}", value, from_unit, result, to_unit);
}