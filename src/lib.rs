use std::fs;

#[allow(dead_code)]
pub fn read_input(filepath: String) -> Vec<String> {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    let m_lines: Vec<String> = contents.lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();
    return m_lines
}

#[allow(dead_code)]
pub fn print_output(part: &str, output: String) {
    println!(" -- Part {}: {}", part, output)
}
