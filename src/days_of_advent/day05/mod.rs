use crate::days_of_advent::common::io;

pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day05");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let highest_seat_id = puzzle_input.lines().map(|l| seat_binary_to_id(&l)).max().unwrap();

    let content = format!(
        "\
        Highest seat ID is {}",
        highest_seat_id,
    );

    let report = io::format_day_report(
        5,
        "Binary Boarding",
        "Find the highest seat ID",
        &content,
    );
    println!("{}", report);
}

fn seat_binary_to_id(binary_string: &str) -> u16 {
    const ROW_CHARS : usize = 7;
    const COL_CHARS: usize = 3;

    let mut itr = binary_string.chars();
    let mut row: u16 = 0;
    for idx in 0..ROW_CHARS {
        row += match itr.nth(0).expect("Not F or B") {
            'B' => 2u16.pow(ROW_CHARS as u32 - idx as u32 - 1),
            _ => 0u16
        };
    }

    let mut col: u16 = 0;
    for idx in (ROW_CHARS)..(ROW_CHARS + COL_CHARS) {
        col += match itr.nth(0).expect("Not L or R") {
            'R' => 2u16.pow((ROW_CHARS + COL_CHARS) as u32 - idx as u32 - 1),
            _ => 0u16
        };
    }

    row * 8 + col
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn acceptance_criteria_seat_id_1() {
        let input = "BFFFBBFRRR";

        let result = seat_binary_to_id(&input);

        assert_eq!(result, 567);
    }

    #[test]
    fn acceptance_criteria_seat_id_2() {
        let input = "BBFFBBFRLL";

        let result = seat_binary_to_id(&input);

        assert_eq!(result, 820);
    }
}