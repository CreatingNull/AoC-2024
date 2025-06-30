#[cfg(test)]
mod tests {

    use std::{process::Command};

    use aoc_lib::aoc_data;

    /// Confirm the binary will execute for all the defined solutions.
    /// No point checking the solutions here, this is just confirming our CLI runner is implemented.
    /// The main point of this is to ensure that I remember to maintain it as complete the days solutions.
    #[test]
    fn test_cli_runner() {
        let bin = env!("CARGO_BIN_EXE_aoc-2024");
        let mut implemented = 0;
        for day in 1..=25 {
            let data_path = aoc_data!(day, "data");
            if data_path.parent().map_or(false, |p| p.exists()) {
                let output = Command::new(bin)
                    .args(["--day", &day.to_string(), "--part", "1", "--data", data_path.to_str().expect("Non-UTF-8 path")])
                    .output()
                    .expect("Failed to execute CLI");
                println!("{:?}", output);
                assert!(output.status.success(), "Failure on day {}", day);
                assert!(String::from_utf8_lossy(&output.stdout).contains("Output: "), "Unexpected output on day {}", day);
                implemented += 1;
            }
        }

        assert!(implemented > 0, "No implemented days detected, probably a developer skill issue.");
    }
}
