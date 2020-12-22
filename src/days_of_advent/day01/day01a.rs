
fn load_input_from_file() -> std::io::Result<String> {
    let cargo_path = env!("CARGO_MANIFEST_DIR");
    let input_file_path = format!("{}/share/days_of_advent/day01/input", cargo_path);

    std::fs::read_to_string(&input_file_path)
}

fn convert_entries_to_i32(entries: &str) -> Vec<i32> {
    return entries.trim().lines().map(|s| s.parse::<i32>().unwrap()).collect();
}

fn repair_report(entries: &[i32], sum: i32) -> Result<i32, &str> {
    for entry in entries {
        let complement = sum - entry;
        if entries.contains(&complement) {
            return Ok(entry * complement);
        }
    }
    Err("No two addends in input summed to 2020")
}

pub fn run() -> () {
    let puzzle_input = load_input_from_file();
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }

    let entries = convert_entries_to_i32(&puzzle_input.unwrap());
    let result = repair_report(&entries, 2020);

    if result.is_err() {
        panic!("Could not repair report: No two entries summed to 2020");
    }

    println!("\tThe result of the input is {}", result.unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_convert_entries_to_i32() {
        let entries = "1\n-1\n0\n11\n99\n2147483647";
        let solution = [1, -1, 0, 11, 99, 2_147_483_647];

        assert_eq!(super::convert_entries_to_i32(&entries), solution);
    }

    #[test]
    fn provided_acceptance_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let solution = 514579;

        let proposed_solution = super::repair_report(&input, 2020).unwrap();

        assert_eq!(proposed_solution, solution);
    }
}