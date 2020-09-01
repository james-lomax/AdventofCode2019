fn mass_to_fuel(mass: i64) -> i64 {
    let m = mass - (mass % 3);
    return m / 3 - 2;    
}

fn fuel_requirement_recursive(mass: i64) -> i64 {
    let f = mass_to_fuel(mass);
    if f < 0 {
        return 0;
    } else {
        // Warning: Rust has no TCO
        return f + fuel_requirement(f);
    }
}

fn fuel_requirement(mass: i64) -> i64 {
    let mut f = mass_to_fuel(mass);
    let mut a = mass_to_fuel(f);
    while a > 0 {
        f += a;
        a = mass_to_fuel(a);
    }
    return f;
}

fn main() {
    println!("12 -> {}", mass_to_fuel(12));
    println!("14 -> {}", mass_to_fuel(14));
    println!("1969 -> {}", mass_to_fuel(1969));

    let contents = std::fs::read_to_string("input.txt").expect("Couldn't read file");

    let total: i64 = contents.split("\n")
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .map(|s| fuel_requirement(s.parse::<i64>().unwrap()))
            .sum();

    // Part 1 answer arrived using mass_to_fuel function directly

    println!("P2 Total: {}", total);
}
