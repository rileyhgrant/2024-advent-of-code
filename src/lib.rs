use std::fs;
use std::time::Instant;

#[allow(dead_code)]
pub fn read_input(filepath: String) -> Vec<String> {
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");
    string_to_vec_string(contents)
}

#[allow(dead_code)]
pub fn string_to_vec_string(input: String) -> Vec<String> {
    let m_lines: Vec<String> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.to_owned())
        .collect();
    return m_lines;
}

#[allow(dead_code)]
pub fn print_output<F>(part: &str, func: F)
where
    F: FnOnce() -> String,
{
    let start = Instant::now();
    let output = func();
    let duration = start.elapsed();
    println!(
        " -- Part {}: {} (took: {:.3}s)",
        part,
        output,
        duration.as_secs_f64()
    )
}

#[allow(dead_code)]
pub fn create_padded_grid(filepath: &str, pad_char: char, pad_amount: usize) -> Vec<Vec<char>> {
    let contents = read_input(format!("input/{}", filepath));

    let line_length = contents.iter().next().unwrap().len();

    let padding_row = vec![vec![pad_char; line_length + pad_amount * 2]; pad_amount];
    let padding_cols = vec![pad_char; pad_amount];

    let mut grid = padding_row.clone();
    contents.iter().for_each(|line| {
        let mut padded_line = padding_cols.clone();
        padded_line.extend(line.chars());
        padded_line.extend(padding_cols.clone());
        grid.push(padded_line);
    });
    grid.extend(padding_row);

    grid
}
