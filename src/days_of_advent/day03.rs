use crate::days_of_advent::common::io;

pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day03");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let slopes = [
        (1,1),
        (1,3),
        (1,5),
        (1,7),
        (2,1)
    ];

    let num_trees = count_trees(&puzzle_input, &(1,3));

    let mult_trees : usize = slopes.iter().map(|s| count_trees(&puzzle_input, s)).product();
    let content = format!("\
        Found {} trees in our path for slope (1.3)\n\
        For each slope, multiplied trees were {}", num_trees, mult_trees);

    let report = io::format_day_report(3, "Toboggan Trajectory", "Count the trees on the slopes", &content);
    println!("{}", report);
}

fn count_trees(map: &str, slope: &(usize, usize)) -> usize {
    let mut pos : (usize, usize) = (0, 0);

    let num_columns = map.lines().next().unwrap().chars().count();
    let mut num_trees : usize = 0;

    while pos.0 < map.lines().count() {
        num_trees += if get_char_at_index(&map, pos).unwrap() == '#' {1} else {0};
        pos = get_index_with_wraparound(&pos, &slope, num_columns);
    }

    num_trees
}

fn get_char_at_index(slopes: &str, pos: (usize, usize)) -> Result<char, &str> {
    if pos.0 >= slopes.lines().count() {
        Err("Given row was out of bounds")
    } else if pos.1 >= slopes.lines().next().unwrap().len() {
        Err("Given column was out of bounds")
    } else {
        let character : char = slopes.lines().nth(pos.0).unwrap().chars().nth(pos.1).unwrap();
        Ok(character)
    }
}

fn get_index_with_wraparound(pos: &(usize, usize), offset: &(usize, usize), col_limit: usize) -> (usize, usize) {
    let mut column = pos.1 + offset.1;
    if column > (col_limit - 1) {
        column -= col_limit;
    }
    (pos.0 + offset.0, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_acceptance_test_3_1() {
        let input = "\
        ..##.......\n\
        #...#...#..\n\
        .#....#..#.\n\
        ..#.#...#.#\n\
        .#...##..#.\n\
        ..#.##.....\n\
        .#.#.#....#\n\
        .#........#\n\
        #.##...#...\n\
        #...##....#\n\
        .#..#...#.#\
        ";

        let num_trees = count_trees(&input, &(1,3));
        assert_eq!(num_trees, 7);
    }

    #[test]
    fn test_grid_slope_1() {
        let new_position = get_index_with_wraparound(&(0,0), &(0,0), 1);
        assert_eq!(new_position, (0,0));
    }

    #[test]
    fn test_grid_slope_2() {
        let new_position = get_index_with_wraparound(&(0,0), &(0,1), 2);
        assert_eq!(new_position, (0,1));
    }

    #[test]
    fn test_grid_slope_3() {
        let new_position = get_index_with_wraparound(&(0,0), &(0,2), 2);
        assert_eq!(new_position, (0,0));
    }

    #[test]
    fn test_grid_slope_4() {
        let new_position = get_index_with_wraparound(&(0,0), &(1,3), 4);
        assert_eq!(new_position, (1,3));
    }

    #[test]
    fn test_get_char_at_index_1() {
        let input = "\
        abdce\n\
        fghij\n\
        klmno\
        ";

        let character = get_char_at_index(&input, (0,0)).unwrap();
        assert_eq!(character, 'a');
    }

    #[test]
    fn test_get_char_at_index_2() {
        let input = "\
        abdce\n\
        fghij\n\
        klmno\
        ";

        let character = get_char_at_index(&input, (2,4)).unwrap();
        assert_eq!(character, 'o');
    }

}