use crate::days_of_advent::common::io;


/// Run the Day 01 puzzle, loading the input from a file, finding entries
/// that sum to a given value, and printing the result of multiplying those
/// entries
pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day01");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }

    let entries = convert_entries_to_i32(&puzzle_input.unwrap());

    const SUM : i32 = 2020;

    let result_2 = repair_report(&entries, SUM, 2);
    let result_3 = repair_report(&entries, SUM, 3);

    let results = String::from(format!(
        "Found two addends that make {}, and they multiply to {}\n\
         Found three addends that make {}, and they multiply to {}",
         SUM.to_string(), result_2.unwrap(), SUM.to_string(), result_3.unwrap()
    ));

    let report = io::format_day_report(1, "Repair Report", "Find entries that add to 2020, and multiply them", &results);
    println!("{}", &report);
}

fn convert_entries_to_i32(entries: &str) -> Vec<i32> {
    return entries.trim().lines().map(|s| s.parse::<i32>().unwrap()).collect();
}

fn repair_report(entries: &[i32], sum: i32, num_to_sum: usize) -> Result<i32, &str> {
    for entry in entries {
        let complement = sum - entry;
        if num_to_sum == 3 {
            match repair_report(entries, complement, 2) {
                Ok(mult) => {
                    return Ok(mult * entry);
                },
                Err(_) => { continue; }
            }
        }
        else {
            if entries.contains(&complement) {
                return Ok(entry * complement);
            }
        }
    }
    Err("No {} addends in input summed to {}")
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

        let proposed_solution = super::repair_report(&input, 2020, 2).unwrap();

        assert_eq!(proposed_solution, solution);
    }
}