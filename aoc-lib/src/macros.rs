/// Display some details on a test run.
///
/// This macro will assert equality and then provide information about the test run.
/// It'll highlight when the data is actual test data, and suppress slightly when the data is example data.
#[macro_export]
macro_rules! assert_pretty_eq {
    ($input:expr, $actual:expr, $expected:expr) => {
        let input_val = $input;
        let actual = $actual;
        let expected = $expected;

        if actual != expected {
            eprintln!("❌ Test failed");
            eprintln!("  📥 input:    {:?}", input_val);
            eprintln!("  🔎 expected: {:?}", expected);
            eprintln!("  🛑 got:      {:?}", actual);
            panic!("Assertion failed");
        } else {
            println!(
                "\x1b[90m✔ verified solution against example.txt ({})\x1b[0m",
                actual
            );
        }
    };
}

/// Generates a path to included data files for a given day.
///
/// Macro takes in the day number and the file name stem, the
/// returns the path to the file.
/// This is to reduce clutter / complexity in test cases.
#[macro_export]
macro_rules! aoc_data {
    ($day:expr, $file_stem:expr) => {{
        use std::path::PathBuf;
        [
            env!("CARGO_MANIFEST_DIR"),
            "src",
            &format!("day{:02}", $day),
            &format!("{}.txt", $file_stem),
        ]
        .iter()
        .collect::<PathBuf>()
    }};
}
