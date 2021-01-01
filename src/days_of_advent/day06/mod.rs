use crate::days_of_advent::common::io;

pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day06");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let groups = split_groups(&puzzle_input);
    let total_any = groups.iter().map(|g| count_unique_letters(&g.to_string())).sum::<usize>();
    let total_all = groups.iter().map(|g| count_consistent_letters(&g.to_string())).sum::<usize>();

    let content = format!(
        "\
        Total answers are {}\n\
        Total consistent answers are {}",
        total_any,
        total_all
    );

    let report = io::format_day_report(
        6,
        "Custom Customs",
        "Count group's yes answers",
        &content,
    );
    println!("{}", report);
}

fn count_unique_letters(group_response: &str) -> usize {
    let individual_responses = group_response.lines();

    let mut sorted = individual_responses.collect::<Vec<&str>>().concat().chars().collect::<Vec<char>>();
    sorted.sort();
    sorted.dedup();

    sorted.len()
}

fn count_consistent_letters(group_response: &str) -> usize {
    let shortest_line = group_response.clone().lines().min_by(|l1, l2| l1.len().cmp(&l2.len())).unwrap();
    let mut num_yes = 0;
    for character in shortest_line.chars() {
        num_yes += if group_response.lines().all(|l| l.contains(character)) {1} else {0};
    }

    num_yes

}

fn split_groups(all_groups: &str) -> Vec<String> {
    all_groups.split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acceptance_test() {
        let input = "\
        abc\n\
        \n\
        a\n\
        b\n\
        c\n\
        \n\
        ab\n\
        ac\n\
        \n\
        a\n\
        a\n\
        a\n\
        a\n\
        \n\
        b";

        let groups = split_groups(&input);
        let total = groups.iter().map(|g| count_unique_letters(&g.to_string())).sum::<usize>();

        assert_eq!(total, 11)
    }

    #[test]
    fn acceptance_test_1() {
        let input = "abc";

        let num_yes = count_unique_letters(&input);

        assert_eq!(num_yes, 3);
    }

    #[test]
    fn acceptance_test_2() {
        let input = "\
        ab\n\
        ac";

        let num_yes = count_unique_letters(&input);

        assert_eq!(num_yes, 3);
    }


}