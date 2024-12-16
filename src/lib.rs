use rand::Rng;
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

pub fn create_padded_grid_from_vec_string(
    vec_string: Vec<String>,
    pad_char: char,
    pad_amount: usize,
) -> Vec<Vec<char>> {
    let line_length = vec_string.iter().next().unwrap().len();

    let padding_row = vec![vec![pad_char; line_length + pad_amount * 2]; pad_amount];
    let padding_cols = vec![pad_char; pad_amount];

    let mut grid = padding_row.clone();
    vec_string.iter().for_each(|line| {
        let mut padded_line = padding_cols.clone();
        padded_line.extend(line.chars());
        padded_line.extend(padding_cols.clone());
        grid.push(padded_line);
    });
    grid.extend(padding_row);

    grid
}

#[allow(dead_code)]
pub fn create_padded_grid(filepath: &str, pad_char: char, pad_amount: usize) -> Vec<Vec<char>> {
    let contents = read_input(format!("input/{}", filepath));
    create_padded_grid_from_vec_string(contents, pad_char, pad_amount)
}

#[allow(dead_code)]
pub fn create_grid(filepath: &str) -> Vec<Vec<char>> {
    create_padded_grid(filepath, '.', 0)
}

#[allow(dead_code)]
pub fn print_grid(grid: &Vec<Vec<char>>, space_before: bool, space_after: bool) {
    if space_before {
        println!("");
    }

    grid.iter().for_each(|line| {
        println!("{}", line.iter().collect::<String>());
    });

    if space_after {
        println!("");
    }
}

fn print_star_pattern(width: usize) {
    let star_size = width / 5;
    if star_size == 0 {
        return;
    }
    if star_size == 1 {
        println!("{:>width$}★", "", width = width);
        return;
    }
    for i in 0..star_size {
        let line_width = 2 * i + 1;
        let line = "★".repeat(line_width);
        println!("{:>width$}{}", "", line, width = width - i);
    }
}

#[allow(dead_code)]
pub fn print_christmas_tree(height: usize) {
    let mut rng = rand::thread_rng();
    let ornaments = vec!['o', '●', '*', '✦', '◆', '❅'];

    println!("");

    print_star_pattern(height);

    for i in 0..height {
        let line_width = 2 * i + 1;
        let num_ornaments = (line_width as f32 * 0.33).round() as usize;

        let symbol = if i == 0 && height > 9 { '★' } else { '*' };

        let mut line: Vec<char> = vec![symbol; line_width];

        let mut ornaments_placed = 0;
        while ornaments_placed < num_ornaments {
            let pos = rng.gen_range(0..line_width);
            if line[pos] == '*' {
                let ornament_idx = rng.gen_range(0..ornaments.len());
                line[pos] = ornaments[ornament_idx];
                ornaments_placed += 1;
            }
        }

        let line_str: String = line.into_iter().collect();
        println!("{:>width$}{}", "", line_str, width = height - i);
    }

    for _ in 0..height / 5 {
        println!("{:>width$}|", "", width = height);
    }

    println!("{:>width$}===", "", width = height - 1);
}
