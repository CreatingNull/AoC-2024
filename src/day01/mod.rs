// --- Day 1: Historian Hysteria ---
use std::collections::HashMap;
use std::path::Path;

use regex::Regex;

use aoc_lib::read_text_file;

/// Part one of the Historian Hysteria task.
///
/// We sort our left and right vectors and compute the sum of the
/// absolute differences between corresponding elements in the two vectors.
///
/// The sum is returned as an `i32`.
pub fn part_one(path: &Path) -> i32 {
    let (mut left, mut right) = convert_data(path);
    // Data needs to be sorted asc.
    left.sort();
    right.sort();
    // Compute the sum of element wise absolute differences.
    let sum: i32 = left.iter().zip(&right).map(|(x, y)| (x - y).abs()).sum();
    sum
}

/// Part two of the Historian Hysteria task.
///
/// We need to count the frequency of elements in each vector.
/// Then for each element in the frequency map of the left vector,
/// we check if it exists in the right vector's frequency map.
/// If it does, we multiply the elements value by the frequency of that element in the right vector,
/// and then multiply that by the frequency of the element in the left vector.read_text_file(path)
///
/// We return the sum of these products as an `i32`, which should be the similarity score.
pub fn part_two(path: &Path) -> i32 {
    let (left, right) = convert_data(path);
    // Count the frequencies, just use a HashMap for this.
    let (left_counts, right_counts) = (count_frequency(&left), count_frequency(&right));
    // Now we compute the similarity score.
    let mut similarity_score: i32 = 0;
    for (value, left_freq) in &left_counts {
        if let Some(right_freq) = right_counts.get(value) {
            similarity_score += value * left_freq * right_freq;
        }
    }
    similarity_score
}

/// Loads the input data and converts it into two vectors of integers.
fn convert_data(path: &Path) -> (Vec<i32>, Vec<i32>) {
    let re: Regex = Regex::new(r"^(\d{1,})\s+(\d{1,})$").unwrap();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());
    for line in read_text_file(path).lines() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Ignore it.
        }
        if let Some(values) = re.captures(trimmed_line) {
            left.push(values.get(1).unwrap().as_str().parse::<i32>().unwrap());
            right.push(values.get(2).unwrap().as_str().parse::<i32>().unwrap());
        }
    }
    (left, right)
}

fn count_frequency(vec: &[i32]) -> HashMap<i32, i32> {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &value in vec {
        *(counts.entry(value)).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rstest::rstest;

    use super::*;
    use aoc_lib::{aoc_data, assert_pretty_eq};

    #[rstest]
    #[case::example_data(&aoc_data!(1, "example"), 11)]
    #[case::real_data(&aoc_data!(1, "data"), 1319616)]
    fn test_part_one(#[case] data: &Path, #[case] sum: i32) {
        assert_pretty_eq!(
            (*data).file_name().unwrap_or_default().to_string_lossy(),
            part_one(data),
            sum
        );
    }

    #[rstest]
    #[case::example_data(&aoc_data!(1, "example"), 31)]
    #[case::real_data(&aoc_data!(1, "data"), 27267728)]
    fn test_part_two(#[case] data: &Path, #[case] sum: i32) {
        assert_pretty_eq!(
            (*data).file_name().unwrap_or_default().to_string_lossy(),
            part_two(data),
            sum
        );
    }
}
