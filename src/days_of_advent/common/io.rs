
// Load the day's input from file
pub fn load_input_from_file(day: &str) -> Result<String, String> {
    let cargo_path = env!("CARGO_MANIFEST_DIR");
    let input_file_path = format!("{}/share/days_of_advent/{}/input", cargo_path, &day);

    match std::fs::read_to_string(&input_file_path) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string())
    }
}

pub fn format_day_report(day_num: usize, day_title: &str, day_description: &str, content: &str) -> String {
    let formatted_content : String = content.lines().map(|s| format!("\t{}\n", s.to_string())).collect();
    format!("Day {}: {}\n{}\n\n{}", day_num, day_title, day_description, formatted_content)
}